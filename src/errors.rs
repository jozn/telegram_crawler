use grammers_client::{AuthorizationError, Client, Config};
use grammers_mtproto::errors::RpcError;
use grammers_mtsender::InvocationError;
use rusqlite;
use serde::export::Formatter;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum GenErr {
    DB(rusqlite::Error),
    Io,
    TGRPC(RpcError),
    TGConnection,
    TGAuth(AuthorizationError),
    TGConverter,
    JSON(serde_json::Error),
}

impl Error for GenErr {}

impl GenErr {
    pub fn is_tg_not_found(&self) -> bool {
        match self {
            GenErr::TGRPC(rpc) => {
                if rpc.code == 400 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_tg_username_free(&self) -> bool {
        match self {
            GenErr::TGRPC(rpc) => {
                if rpc.code == 400 && &rpc.name == "USERNAME_NOT_OCCUPIED" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl Display for GenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use GenErr::*;
        // match *self {
        //
        // };
        // let s = format!("{}")
        write!(f, "Gen err: {:?}", self)
    }
}

impl From<AuthorizationError> for GenErr {
    fn from(tg_err: AuthorizationError) -> Self {
        use AuthorizationError::*;
        match &tg_err {
            Invocation(inv) => match inv {
                InvocationError::RPC(rpc) => GenErr::TGRPC(rpc.clone()),
                _ => GenErr::TGConnection,
            },
            IO(io) => GenErr::TGAuth(tg_err),
            Gen(gen) => GenErr::TGAuth(tg_err),
        }
    }
}

impl From<InvocationError> for GenErr {
    fn from(inv: InvocationError) -> GenErr {
        match inv {
            InvocationError::RPC(rpc) => GenErr::TGRPC(rpc.clone()),
            _ => GenErr::TGConnection,
        }
    }
}

impl From<rusqlite::Error> for GenErr {
    fn from(e: rusqlite::Error) -> GenErr {
        GenErr::DB(e)
    }
}

impl From<serde_json::Error> for GenErr {
    fn from(e: serde_json::Error) -> GenErr {
        GenErr::JSON(e)
    }
}
