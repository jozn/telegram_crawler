#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(soft_unstable)]

use async_std::task;
use grammers_client::{AuthorizationError, Client, Config};
use grammers_tl_types as tl;
use grammers_session as session;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};

mod types;
mod tg;

fn read() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => panic!("Can not get input value: {:?}", e)
    }
}

async fn async_main() -> Result<(), AuthorizationError> {
    println!("Connecting to Telegram...");
    let api_id = 123259;
    let api_hash = "e88ec58aa1ce01f5630e194e9571d751".to_string();
    let cf = Config{
        session: session::Session::load_or_create("./s1.session").unwrap() ,
        api_id: api_id,
        api_hash: api_hash.clone(),
        params: Default::default()
    };
    let mut client = Client::connect(cf).await?;
    println!("Connected!");

    println!("Sending ping...");
    dbg!(client.invoke(&tl::functions::Ping { ping_id: 90 }).await?);
    println!("Ping sent successfully!");

    // login
    if !client.is_authorized().await? {
        println!("Signing in...");
        let phone = "989338828058";
        match client.request_login_code(phone,api_id, &api_hash).await {
            Ok(res) => {
                println!("write the code form telgeram ....");
                let s = read();
                match client.sign_in(&s).await {
                    Ok(user) => {
                        println!("sigin in {:?} ", user)
                    },
                    Err(err) => {
                        println!("sigin in error {:?} ", err)
                    },
                    _ => {}
                }
            },
            Err(e) => {
                println!("error in sending conde: {}", e);
            }
        };
        // client.bot_sign_in(&token, api_id, &api_hash).await?;
        println!("Signed in!");
    } else {
        println!("!! Already Signed in!");

    }

    run2(client).await;

    Ok(())
}

fn main() -> Result<(), AuthorizationError> {
    task::block_on(async_main())
}

use std::sync::{Arc, Mutex};
async fn run2( mut c: Client){

    let mut app = types::App{
        login: vec![],
        channels: Default::default(),
        sessions: vec![],
        dcs: vec![],
        client: c,
    };
    // let mut app = Arc::new(Mutex::new(app));
    // let app1 = app.get_mut().unwrap();

    // app.get_dialogs().await;
    tg::get_messages().await;
    // app.get_channel_info().await;
    // app.get_channel_by_username().await;
    // app.get_chat_id().await;

}
