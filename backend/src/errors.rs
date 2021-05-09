use crate::code_loc;
use actix_web::client::SendRequestError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct MyError {
    pub(crate) err: anyhow::Error,
}
impl actix_web::error::ResponseError for MyError {}
impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> MyError {
        MyError { err }
    }
}

impl MyError {
    pub fn new<M>(message: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self {
            err: anyhow::Error::msg(message),
        }
    }
}
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}
impl Error for MyError {}
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        Self {
            err: anyhow::Error::from(err),
        }
    }
}
impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            err: anyhow::Error::from(err),
        }
    }
}
impl From<SendRequestError> for MyError {
    fn from(err: SendRequestError) -> Self {
        let e = match err {
            SendRequestError::Url(e) => {
                format!("{:?}", e)
            }
            SendRequestError::Connect(e) => {
                format!("{:?}", e)
            }
            SendRequestError::Send(e) => e.to_string(),
            SendRequestError::Response(e) => {
                format!("{:?}", e)
            }
            SendRequestError::Http(e) => format!("{:?}\n{:?}", code_loc!(), e),
            SendRequestError::H2(e) => format!("{:?}\n{:?}", code_loc!(), e),
            SendRequestError::Body(e) => format!("{:?}\n{:?}", code_loc!(), e),
            e => {
                format!("{:?}", e)
            }
        };
        Self::new(e)
    }
}
// impl From<rusqlite::Error> for MyError {
//     fn from(e: rusqlite::Error) -> Self {
//         match e {
//             err => MyError::new(format!("{:?}\n{:?}", code_loc!(), err)),
//         }
//     }
// }
