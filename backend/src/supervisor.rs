//! Simple websocket client.
use crate::server::routes::CLIENT_PORT;
// use actix::*;
use log::{debug, error};
use rust_ac::websocket::{
    header::{Header, HeaderFormat, Headers},
    {ClientBuilder, Message, OwnedMessage},
};
use std::sync::mpsc::channel;
use std::{
    error::Error,
    fmt::Formatter,
    sync::mpsc::{Iter, Receiver, RecvError, RecvTimeoutError, SendError, Sender, TryRecvError},
    thread,
    thread::JoinHandle,
    time::Duration,
};

pub struct Supervisor {}
pub struct SupervisorChannel {
    supervisor_sender: Sender<String>,
    server_receiver: Receiver<String>,
    _join_handle: JoinHandle<()>,
}
impl SupervisorChannel {
    pub fn send(&mut self, txt: String) -> Result<(), SendError<String>> {
        self.supervisor_sender.send(txt)
    }
    pub fn recv(&mut self) -> Result<String, RecvError> {
        self.server_receiver.recv()
    }
    #[allow(dead_code)]
    pub fn try_recv(&mut self) -> Result<String, TryRecvError> {
        self.server_receiver.try_recv()
    }
    pub fn iter(&mut self) -> Iter<'_, String> {
        self.server_receiver.iter()
    }
    pub fn recv_timeout(&mut self, secs: u64) -> Result<String, RecvTimeoutError> {
        self.server_receiver.recv_timeout(Duration::from_secs(secs))
    }
}

impl Supervisor {
    pub fn connect() -> Result<SupervisorChannel, Box<dyn Error>> {
        let (supervisor_sender, supervisor_receiver) = channel::<String>();
        let (server_sender, server_receiver) = channel::<String>();
        let mut headers = Headers::new();
        headers.set(SupervisorHeader {
            string: "false".to_string(),
        });
        let client = ClientBuilder::new(&format!("ws://127.0.0.1:{}", CLIENT_PORT))
            .map_err(|e| e.to_string())?
            .custom_headers(&headers)
            .connect_insecure()
            .map_err(|x| {
                error!("{:?}", x);
                x
            })?;
        let (mut receiver, mut sender) = client.split().unwrap();
        let _l = thread::spawn(move || {
            while let Ok(OwnedMessage::Text(msg)) = receiver.recv_message() {
                debug!("To_Send: {}", msg);
                if let Err(e) = server_sender.send(msg) {
                    error!("{}", e.to_string())
                };
            }
        });
        let join_handle = thread::spawn(move || loop {
            if let Ok(msg) = supervisor_receiver.try_recv() {
                if msg == "Disconnect" {
                    if let Err(e) = sender.shutdown_all() {
                        error!("{:?}", e.to_string());
                    }
                    break;
                }
                debug!("Received: {}", msg);
                if let Err(e) = sender.send_message(&Message::text(msg)) {
                    error!("{}", e.to_string())
                };
            }
        });

        Ok(SupervisorChannel {
            supervisor_sender,
            server_receiver,
            _join_handle: join_handle,
        })
    }
}

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
