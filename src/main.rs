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
use grammers_tl_types::enums::Message;

mod types;

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

    let mut app = App{
        login: vec![],
        channels: Default::default(),
        sessions: vec![],
        dcs: vec![],
        client: c,
    };

    let mut app = Arc::new(Mutex::new(app));
    let app1 = app.get_mut().unwrap();

}



// types
use std::collections::HashMap;
pub struct App {
    login: Vec<LoginPhone>,
    channels: HashMap<i64,ChannelSpace>,
    sessions: Vec<Session>,
    dcs: Vec<DC>,
    client: Client,

}

#[derive(Clone, Debug)]
pub struct MsgReplayTo {

}

#[derive(Clone, Debug)]
pub struct MsgForwarded {

}

#[derive(Clone, Debug)]
pub struct Msg {

    raw: tl::types::Message,
}

#[derive(Clone, Debug)]
pub enum MediaType {

}

#[derive(Clone, Debug)]
pub struct Media {

}

#[derive(Clone, Debug)]
pub struct ChannelSpace {
    info: ChannelInfo,
    msgs: HashMap<u32,Msg>,

}

#[derive(Clone, Debug)]
pub struct ChannelInfo {

}

#[derive(Clone, Debug)]
pub struct DC {

}

#[derive(Clone, Debug)]
pub struct Session {

}

#[derive(Clone, Debug)]
pub struct  LoginPhone {

}
/////////////////////////////////////////

impl App {
    pub fn get_contacts(&mut self) {
        // get contacts
        let request = tl::functions::contacts::GetContacts {
            hash:23,
        };
        let mt : tl::enums::contacts::Contacts = self.client.invoke(&request).await.unwrap();
        // println!("contacts {:#?}", mt);
    }

    pub fn get_dialogs(&mut self) {
        // get dialogs
        // let id = 754247155;
        let request = tl::functions::messages::GetDialogs {
            exclude_pinned: false,
            folder_id: None,
            offset_date: 0,
            offset_id: 0,
            offset_peer: tl::types::InputPeerEmpty {}.into(),
            limit: 50,
            hash: 0,
        };
        let mt : tl::enums::messages::Dialogs = c.invoke(&request).await.unwrap();
        // println!("dilagos {:#?}", mt);
    }

    pub fn get_messages(&mut self) {
        let request = tl::functions::messages::GetHistory {
            /*peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1072723547,
                access_hash: -1615658883512673699,
            }),*/
            peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1355843251,
                access_hash: -6028453276089081451,
            }),
            offset_id: 0,
            offset_date: 0,
            add_offset: 0,
            limit: 2,
            max_id: 0,
            min_id: 0,
            hash: 0
        };

        let mt : tl::enums::messages::Messages = c.invoke(&request).await.unwrap();
    }

    pub fn process_messages(&mut self, mt: tl::enums::messages::Messages) {
        match mt {
            Messages::ChannelMessages(m) => {
                for m in m.messages {
                    match m {
                        Message::Message(m2) => {

                            println!(">>> \n {:#?}", m2);
                            if let Some(f) = m2.media {
                                println!("{:?}", f)
                            }

                        },
                        _ => {}
                    }

                }
            },
            _ => {
                println!("other form of messages!")
            }
        }
    }

    pub fn bench_messages_loading_flood(&mut self) {
        let request = tl::functions::messages::GetHistory {
            peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1355843251,
                access_hash: -6028453276089081451,
            }),
            offset_id: 0,
            offset_date: 0,
            add_offset: 0,
            limit: 2,
            max_id: 0,
            min_id: 0,
            hash: 0
        };

        let mt : tl::enums::messages::Messages = c.invoke(&request).await.unwrap();

        let mut cnt = 0;
        for i in 1..500 {
            println!("> {} -- ", i);
            let mt : tl::enums::messages::Messages = c.invoke(&request).await.unwrap();

            match mt {
                Messages::ChannelMessages(m) => {
                    for m in m.messages {
                        match m {
                            Message::Message(m2) => {
                                cnt += 1;
                                // println!("{:?}", m2)
                                println!("{}", cnt)
                            },
                            _ => {}
                        }

                    }
                },
                _ => {
                    println!("other form of messages!")
                }
            }
        }
    }
}

/////////////////////////////////////////
async fn run( mut c: Client){



    // get dialogs
    // let id = 754247155;
    let request = tl::functions::messages::GetDialogs {
        exclude_pinned: false,
        folder_id: None,
        offset_date: 0,
        offset_id: 0,
        offset_peer: tl::types::InputPeerEmpty {}.into(),
        limit: 50,
        hash: 0,
    };
    let mt : tl::enums::messages::Dialogs = c.invoke(&request).await.unwrap();
    // println!("dilagos {:#?}", mt);

    // /*// get chats
    // let id = 754247155;
    let request = tl::functions::messages::GetHistory {
        /*peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
            channel_id: 1072723547,
            access_hash: -1615658883512673699,
        }),*/
        peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
            channel_id: 1355843251,
            access_hash: -6028453276089081451,
        }),
        offset_id: 0,
        offset_date: 0,
        add_offset: 0,
        limit: 2,
        max_id: 0,
        min_id: 0,
        hash: 0
    };

    let mt : tl::enums::messages::Messages = c.invoke(&request).await.unwrap();

    match mt {
        Messages::ChannelMessages(m) => {
            for m in m.messages {
                match m {
                    Message::Message(m2) => {

                        println!(">>> \n {:#?}", m2);
                        if let Some(f) = m2.media {
                            println!("{:?}", f)
                        }

                    },
                    _ => {}
                }

            }
        },
        _ => {
            println!("other form of messages!")
        }
    }

    let mut cnt = 0;
    /*for i in 1..500 {
        println!("> {} -- ", i);
        let mt : tl::enums::messages::Messages = c.invoke(&request).await.unwrap();

        match mt {
            Messages::ChannelMessages(m) => {
                for m in m.messages {
                    match m {
                        Message::Message(m2) => {
                            cnt += 1;
                            // println!("{:?}", m2)
                            println!("{}", cnt)
                        },
                        _ => {}
                    }

                }
            },
            _ => {
                println!("other form of messages!")
            }
        }
    }*/


    // println!("messages {:#?}", mt);
    // println!("messages {:?}", mt);

}