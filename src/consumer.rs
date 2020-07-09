use grammers_client::{AuthorizationError, Client, Config};

use async_std::task;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crossbeam::channel::bounded;

use crate::{con_mgr, types};

struct Consumer {
    client: Client,
}

lazy_static! {

    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);

}

impl Consumer {
    pub fn new() -> Consumer {
        let c = task::block_on(con_mgr::get_new_session()).unwrap();
        // let c = con_mgr::get_new_session().await.unwrap();
        Self { client: c }
    }
}

pub fn start_new_consumer() -> thread::JoinHandle<()> {
    let th = thread::spawn(|| {
        let cons = Consumer::new();

        let (sReqSyncChannel, rReqSyncChannel) = bounded::<types::ReqSyncChannel>(10);
        let (sResSyncChannel, rResSyncChannel) = bounded::<types::ResSyncChannel>(10);
        let (sReqSyncMessages, rReqSyncMessages) = bounded::<types::ReqSyncMessages>(10);
        let (sResSyncMessages, rResSyncMessages) = bounded::<types::ResSyncMessages>(10);
        let (sReqResolveUsername, rReqResolveUsername) = bounded::<types::ReqResolveUsername>(10);
        let (sResResolveUsername, rResResolveUsername) = bounded::<types::ResResolveUsername>(10);

        let ms = |ms| Duration::from_millis(ms);

        loop {
            let c = crossbeam::channel::tick(ms(500));
            // println!("1 ");
            c.recv();
            println!("tock!");
        }
    });
    th
}
