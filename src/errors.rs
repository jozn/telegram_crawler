use grammers_client::{AuthorizationError, Client, Config};
use grammers_mtproto::errors::RpcError;
use grammers_mtsender::InvocationError;
use serde::export::Formatter;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
enum GenErr {
    DB,
    Io,
    TGRPC(RpcError),
    TGAuth(AuthorizationError),
    Other(),
}

impl Error for GenErr {}

impl Display for GenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use GenErr::*;
        // match *self {
        //
        // };
        write!(f, "gen err error")
    }
}

impl From<AuthorizationError> for GenErr {
    fn from(tg_err: AuthorizationError) -> Self {
        use AuthorizationError::*;
        match &tg_err {
            Invocation(inv) => match inv {
                InvocationError::RPC(rpc) => GenErr::TGRPC(rpc.clone()),
                _ => GenErr::TGAuth(tg_err),
            },
            IO(io) => GenErr::TGAuth(tg_err),
            Gen(gen) => GenErr::TGAuth(tg_err),
        }
    }
}
