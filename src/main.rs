#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

#[macro_use]
extern crate lazy_static;

use async_std::task;
use grammers_client::{AuthorizationError, Client, Config};
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};

mod client_pool;
mod types;

use std::cell::*;

mod tg_old;
mod tg2;
mod crawl;
mod db;
mod con_mgr;
mod consumer;
mod tg;
mod pipe;

// mod threading;

fn main() {
    task::block_on(crawl::run())
}
/*
use std::sync::{Arc, Mutex};
async fn run2() {
    let mut app = types::App {
        login: vec![],
        channels: Default::default(),
        sessions: vec![],
        dcs: vec![],
        clients: Mutex::new(Cell::new(client_pool::ClientPool { client: None })),
    };

    let app2 = Arc::new(app);
    tg::get_contacts(&app2).await;
    tg::get_messages(&app2).await;

    // threading::run();


    // tg2::get_contacts( app2.clone()).await;
    // tg::get_contacts(&app2).await;
    // tg::get_messages(&app2).await;
    // let mut app = Arc::new(Mutex::new(app));
    // let app1 = app.get_mut().unwrap();

    // app.get_dialogs().await;
    // tg::get_messages().await;
    // app.get_channel_info().await;
    // app.get_channel_by_username().await;
    // app.get_chat_id().await;
}
*/