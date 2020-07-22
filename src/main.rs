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

mod con_mgr;
mod config;
mod crawl;
mod db;
mod errors;
// mod tg_old;
mod utils;
mod tg;

#[tokio::main]
async fn main() {
    // crawl::crawl_next_username().await;
    // crawl::crawl_config().await;
    // crawl::crawl_next_channel().await;

    // tg::get_file()
    for i in 0..1 {
        println!("{}", 1);
        // crawl::crawl_next_username().await;
        let r = crawl::crawl_next_channel_messages().await;
        println!("{} {:?}", i, r);
    }
}
