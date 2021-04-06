//! Simple websocket client.
use std::thread;
use std::time::Duration;

use crate::routes::CLIENT_PORT;
use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use actix_web::rt::{Arbiter, System};

use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};

use actix_web::web::Bytes;
use crossbeam::channel::{Sender, TryRecvError};
use futures::stream::{SplitSink, StreamExt};

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

pub struct Supervisor {
    sink_write: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
}
impl Supervisor {
    pub fn connect() -> Sender<String> {
        let (sender, receiver) = crossbeam::channel::unbounded();
        Arbiter::spawn(async {
            Client::builder()
                .timeout(Duration::from_secs(15))
                .finish()
                .ws(format!("ws://127.0.0.1:{}", CLIENT_PORT))
                .header("supervisor", "true")
                .connect()
                .await
                .map_err(|x| println!("{:?}", x))
                .map(|(_response, frame)| {
                    let (sink, stream) = frame.split();
                    let addr = Supervisor::create(|ctx| {
                        Supervisor::add_stream(stream, ctx);
                        Supervisor {
                            sink_write: SinkWrite::new(sink, ctx),
                        }
                    });
                    thread::spawn(move || loop {
                        if let Ok(msg) = receiver.try_recv().map_err(|e| match e {
                            TryRecvError::Empty => {}
                            TryRecvError::Disconnected => {
                                println!("Disconnected")
                            }
                        }) {
                            addr.do_send(ClientCommand(msg))
                        }
                    })
                });
        });
        sender
    }
}
impl Actor for Supervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

impl Supervisor {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(5, 0), |act, ctx| {
            act.sink_write.write(Message::Ping(Bytes::from_static(b"")));
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for Supervisor {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        println!("sending message:{}", &msg.0);
        self.sink_write.write(Message::Text(msg.0));
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for Supervisor {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            println!("Server: {:?}", txt)
        } else if let Ok(Frame::Pong(_)) = msg {
            println!("Server: Pong")
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for Supervisor {}
