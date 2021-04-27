//! Simple websocket client.
use crate::routes::CLIENT_PORT;
use actix::*;
use crossbeam::channel::Iter;
use log::error;
use rust_ac::websocket::header::{Header, HeaderFormat, Headers};
use rust_ac::websocket::{ClientBuilder, Message, OwnedMessage};
use std::error::Error;
use std::fmt::Formatter;
use std::thread;
use std::thread::JoinHandle;

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

pub struct Supervisor {
    // sink_write: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
// server_sender: crossbeam::channel::Sender<String>,
}
pub struct SupervisorChannel {
    supervisor_sender: crossbeam::channel::Sender<String>,
    server_receiver: crossbeam::channel::Receiver<String>,
    join_handle: JoinHandle<()>,
}
impl SupervisorChannel {
    pub fn send(&mut self, txt: String) -> Result<(), crossbeam::channel::SendError<String>> {
        self.supervisor_sender.send(txt)
    }
    pub fn recv(&mut self) -> Result<String, crossbeam::channel::RecvError> {
        self.server_receiver.recv()
    }
    #[allow(dead_code)]
    pub fn try_recv(&mut self) -> Result<String, crossbeam::channel::TryRecvError> {
        self.server_receiver.try_recv()
    }
    pub fn iter(&mut self) -> Iter<'_, String> {
        self.server_receiver.iter()
    }
}

impl Supervisor {
    pub fn connect() -> Result<SupervisorChannel, Box<dyn Error>> {
        let (supervisor_sender, supervisor_receiver) = crossbeam::channel::unbounded::<String>();
        let (server_sender, server_receiver) = crossbeam::channel::unbounded::<String>();
        let mut headers = Headers::new();
        headers.set(SupervisorHeader {
            string: "false".to_string(),
        });
        let client = ClientBuilder::new(&format!("ws://127.0.0.1:{}", CLIENT_PORT))
            .map_err(|e| e.to_string())?
            .custom_headers(&headers)
            .connect_insecure()
            .map_err(|x| {
                println!("{:?}", x);
                x
            })?;
        let (mut receiver, mut sender) = client.split().unwrap();
        let _l = thread::spawn(move || {
            while let Ok(OwnedMessage::Text(msg)) = receiver.recv_message() {
                println!("To_Send: {}", msg);
                if let Err(e) = server_sender.send(msg) {
                    error!("{}", e.to_string())
                };
            }
        });
        let join_handle = thread::spawn(move || loop {
            if let Ok(msg) = supervisor_receiver.try_recv() {
                if msg == "Disconnect" {
                    sender.shutdown_all();
                    break;
                }
                println!("Received: {}", msg);
                if let Err(e) = sender.send_message(&Message::text(msg)) {
                    error!("{}", e.to_string())
                };
            }
        });

        // .map(|mut e| {
        //     while let Ok(msg) = supervisor_receiver.recv() {
        //         e.send_message(&Message::Text(msg))
        //     }
        // });
        // ClientBuilder::new(&format!("ws://127.0.0.1:{}", CLIENT_PORT)).map_err(|e|e.to_string())?.custom_headers()
        //     Client::builder()
        //         .timeout(Duration::from_secs(15))
        //         .finish()
        //         .ws(format!("ws://127.0.0.1:{}", CLIENT_PORT))
        //         .header("supervisor", "true")
        //         .connect()
        //         .await
        //         .map_err(|x| println!("{:?}", x))
        //         .map(|(_response, frame)| {
        //             let (sink, stream) = frame.split();
        //             let addr = Supervisor::create(|ctx| {
        //                 Supervisor::add_stream(stream, ctx);
        //                 Supervisor {
        //                     sink_write: SinkWrite::new(sink, ctx),
        //                     server_sender,
        //                 }
        //             });
        //             let join_handle = thread::spawn(move || loop {
        //                 while let Ok(msg) = supervisor_receiver.try_recv() {
        //                     addr.do_send(ClientCommand(msg));
        //                 }
        //             })
        //         })

        Ok(SupervisorChannel {
            supervisor_sender,
            server_receiver,
            join_handle,
        })
    }
}

// impl Actor for Supervisor {
//     type Context = Context<Self>;
//
//     fn started(&mut self, ctx: &mut Context<Self>) {
//         // start heartbeats otherwise server will disconnect after 10 seconds
//         self.hb(ctx)
//     }
//
//     fn stopped(&mut self, _: &mut Context<Self>) {
//         debug!("Server Disconnected");
//
//         // Stop application on disconnect
//         System::current().stop();
//     }
// }
//
// impl Supervisor {
//     fn hb(&self, ctx: &mut Context<Self>) {
//         ctx.run_later(Duration::new(5, 0), |act, ctx| {
//             act.sink_write.write(Message::Ping(Bytes::from_static(b"")));
//             act.hb(ctx);
//
//             // client should also check for a timeout here, similar to the
//             // server code
//         });
//     }
// }
//
// /// Handle stdin commands
// impl Handler<ClientCommand> for Supervisor {
//     type Result = ();
//
//     fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
//         trace!("sending message:{}", &msg.0);
//         self.sink_write.write(Message::Text(msg.0));
//     }
// }
//
// /// Handle server websocket messages
// impl StreamHandler<Result<Frame, WsProtocolError>> for Supervisor {
//     fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
//         if let Ok(Frame::Text(txt)) = msg {
//             trace!("Server:{:?}", txt);
//             self.server_sender
//                 .send(String::from_utf8(txt.to_vec()).unwrap())
//                 .unwrap();
//         } else if let Ok(Frame::Pong(_)) = msg {
//             trace!("Server: Pong")
//         }
//     }
//
//     fn started(&mut self, _ctx: &mut Context<Self>) {
//         debug!("StreamHandler Connected");
//     }
//
//     fn finished(&mut self, ctx: &mut Context<Self>) {
//         debug!("StreamHandler disconnected");
//         ctx.stop()
//     }
// }
//
// impl actix::io::WriteHandler<WsProtocolError> for Supervisor {}

#[derive(Clone, Debug)]
struct SupervisorHeader {
    string: String,
}

impl Header for SupervisorHeader {
    fn header_name() -> &'static str {
        "supervisor"
    }

    fn parse_header(_raw: &[Vec<u8>]) -> rust_ac::websocket::header::Result<Self> {
        Ok(Self {
            string: "true".to_string(),
        })
    }
}

impl HeaderFormat for SupervisorHeader {
    fn fmt_header<'a>(&self, f: &mut Formatter<'a>) -> core::fmt::Result {
        f.write_str("Supervisor: false")
    }
}
