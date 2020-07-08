
use crossbeam::channel::bounded;
use crossbeam::channel::{Sender,Receiver};

use crate::{types, tg_old, client_pool, dbi, consumer};

lazy_static! {

    pub static ref SEND_CHENNEL: (Sender<types::ReqSyncChannel>,Receiver<types::ReqSyncChannel>) = bounded::<types::ReqSyncChannel>(10);

}


macro_rules! new_ch {
    ($ch:ident) => {
        lazy_static! {
            pub static ref $ch: (Sender<types::$ch >,Receiver<types::$ch>) = bounded::<types::$ch>(10);
        }
    };
}

new_ch!(ReqSyncChannel);
new_ch!(ResSyncChannel);
new_ch!(ReqSyncMessages);
new_ch!(ResSyncMessages);
new_ch!(ReqResolveUsername);
new_ch!(ResResolveUsername);

// let (sReqSyncChannel,rReqSyncChannel) = bounded::<types::ReqSyncChannel>(10);
// let (sResSyncChannel,rResSyncChannel) = bounded::<types::ResSyncChannel>(10);
// let (sReqSyncMessages,rReqSyncMessages) = bounded::<types::ReqSyncMessages>(10);
// let (sResSyncMessages,rResSyncMessages) = bounded::<types::ResSyncMessages>(10);
// let (sReqResolveUsername,rReqResolveUsername) = bounded::<types::ReqResolveUsername>(10);
// let (sResResolveUsername,rResResolveUsername) = bounded::<types::ResResolveUsername>(10);