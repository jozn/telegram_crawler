use async_std::task;
use grammers_client::{AuthorizationError, Client, Config};
use grammers_mtsender::InvocationError;
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};
use grammers_tl_types::RemoteCall;
use std::io::Write;

use crate::{types, types::Caller, errors::GenErr, utils};
use crate::{types::{Media,MediaThumb}};
use crate::{tg::converter};

use log::kv::Source;

#[derive(Clone, Debug)]
pub struct ReqGetMessages {
    pub channel_id: i32,
    pub access_hash: i64,
    pub offset_id: i32,
    pub offset_date: i32,
    pub add_offset: i32,
    pub limit: i32,
    pub max_id: i32,
    pub min_id: i32,
    pub hash: i32,
}

#[derive(Clone, Debug)]
pub struct MsgHolder {
    pub msgs: Vec<types::Msg>,
    pub channels: Vec<types::ChannelInfo>,
    pub urls: Vec<String>,
    pub users: Vec<String>,
}

pub async fn get_configs(caller: &mut Caller) -> Result<tl::enums::Config, GenErr> {
    let request = tl::functions::help::GetConfig {};
    let res = caller.client.invoke(&request).await?;
    println!("config {:#?}", res);
    Ok(res)
}

pub async fn get_channel_info(
    caller: &mut Caller,
    channel_id: i32,
    access_hash: i64,
) -> Result<types::ChannelInfo, GenErr> {
    let request = tl::functions::channels::GetFullChannel {
        channel: tl::enums::InputChannel::Channel(tl::types::InputChannel {
            channel_id: channel_id,
            access_hash: access_hash,
        }),
    };
    let res = caller.client.invoke(&request).await?;

    let mut ci = types::ChannelInfo::default();

    use tl::enums::messages::ChatFull;
    match res {
        ChatFull::Full(full) => {
            use tl::enums::ChatFull;
            match full.full_chat {
                ChatFull::ChannelFull(c) => {
                    ci.id = c.id;
                    ci.pts = c.pts;
                    ci.read_inbox_max_id = c.read_inbox_max_id;
                    ci.members_count = c.participants_count.unwrap_or(0);
                }
                _ => {}
            }

            if full.chats.len() == 1 {
                let chat = full.chats.get(0).unwrap();

                use tl::enums::Chat;
                match chat {
                    Chat::Channel(ch) => {
                        ci.id = ch.id;
                        ci.title = ch.title.clone();
                        ci.username = ch.username.clone().unwrap_or("".to_string());
                        ci.access_hash = ch.access_hash.unwrap_or(0);
                        ci.date = ch.date;
                        ci.version = ch.version;
                        // ci.members_count = ch.participants_count.unwrap_or(0); // Note: it is None in here! use 'full_chat'
                        ci.restricted = ch.restricted;
                        ci.megagroup = ch.megagroup;
                        ci.full_data = true;
                    }
                    _ => {}
                };
                println!("channel info {:#?}", ci);
                return Ok(ci);
            }
        }
    }

    Err(GenErr::TgConverter)
}

pub async fn get_channel_by_username(
    caller: &mut Caller,
    username: &str,
) -> Result<types::ChannelByUsernameResult, GenErr> {
    let request = tl::functions::contacts::ResolveUsername {
        username: username.to_string(),
    };
    let res = caller.client.invoke(&request).await?;
    // println!("resolve username:  {:#?}", res);

    use tl::enums::contacts::ResolvedPeer;
    match res {
        ResolvedPeer::Peer(peer) => {
            use tl::enums::Chat;
            for chat in peer.chats {
                match chat {
                    Chat::Channel(channel) => {
                        let c = channel;
                        let res = types::ChannelByUsernameResult {
                            id: c.id,
                            title: c.title,
                            username: c.username.unwrap_or("".to_string()),
                            access_hash: c.access_hash.unwrap_or(0),
                            date: c.date,
                            photo: 0,
                            version: c.version,
                            restricted: c.restricted,
                            megagroup: c.megagroup,
                        };
                        return (Ok(res));
                        // println!(">>> channel: #{:#?} ", res);
                    }
                    _ => {}
                }
            }
        }
    }
    Err(GenErr::TgConverter)
}

pub async fn get_messages(caller: &mut Caller, req: ReqGetMessages) -> Result<MsgHolder, GenErr> {
    let request = tl::functions::messages::GetHistory {
        peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel {
            channel_id: req.channel_id,
            access_hash: req.access_hash,
        }),
        offset_id: req.offset_id,
        offset_date: req.offset_date,
        add_offset: req.add_offset,
        limit: req.limit, //100
        max_id: req.max_id,
        min_id: req.min_id,
        hash: req.hash,
    };

    // let mt: tl::enums::messages::Messages = send_req(g, &request).await?;
    let mt: tl::enums::messages::Messages = caller.client.invoke(&request).await?;
    // println!("messages #{:#?}", mt);
    process_channel_msgs(caller, mt).await
}

async fn process_channel_msgs(
    caller: &mut Caller,
    mt: tl::enums::messages::Messages,
) -> Result<MsgHolder, GenErr> {
    let mut msg_holder = MsgHolder {
        msgs: vec![],
        channels: vec![],
        urls: vec![],
        users: vec![],
    };

    // let mut msgs = vec![];
    // let mut urls: Vec<String> = vec![];
    match mt {
        Messages::ChannelMessages(cm) => {
            println!("messages #{:#?}", cm);
            msg_holder.channels = converter::process_inline_channel_chats(cm.chats.clone());
            converter::process_inline_channel_users(&cm.users);

            let res = converter::process_inline_channel_messages(cm.messages.clone());
            msg_holder.msgs = res.0;
            msg_holder.urls = res.1;
        }
        _ => println!("other form of messages!"),
    };
    Ok(msg_holder)
    // println!("msgs {:#?} ", msgs);
    // println!("urls {:#?} ", urls);
}

////////////////////////////////////// Archives ////////////////////////////////////

async fn get_contacts(g: &types::G) {
    let request = tl::functions::contacts::GetContacts { hash: 23 };
    let mt: tl::enums::contacts::Contacts = send_req_dep(g, &request).await.unwrap();
    // println!("contacts {:#?}", mt);
}

async fn get_dialogs(g: &types::G) {
    let request = tl::functions::messages::GetDialogs {
        exclude_pinned: false,
        folder_id: None,
        offset_date: 0,
        offset_id: 0,
        offset_peer: tl::types::InputPeerEmpty {}.into(),
        limit: 50,
        hash: 0,
    };
    let mt: tl::enums::messages::Dialogs = send_req_dep(g, &request).await.unwrap();
    // println!("dilagos {:#?}", mt);
}

async fn get_chat_id(g: &types::G) {
    let request = tl::functions::contacts::GetContactIds { hash: 1149267300 };
    let res = send_req_dep(g, &request).await.unwrap();
    // println!("get_chat_id:  {:#?}", res);
}

async fn bench_messages_loading_flood(g: &types::G) {
    let request = tl::functions::messages::GetHistory {
        peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel {
            channel_id: 1355843251,
            access_hash: -6028453276089081451,
        }),
        offset_id: 0,
        offset_date: 0,
        add_offset: 0,
        limit: 2,
        max_id: 0,
        min_id: 0,
        hash: 0,
    };

    let mt: tl::enums::messages::Messages = send_req_dep(g, &request).await.unwrap();

    let mut cnt = 0;
    for i in 1..500 {
        // println!("> {} -- ", i);
        let mt: tl::enums::messages::Messages = send_req_dep(g, &request).await.unwrap();

        match mt {
            Messages::ChannelMessages(m) => {
                for m in m.messages {
                    match m {
                        Message::Message(m2) => {
                            cnt += 1;
                            // println!("{:?}", m2)
                            println!("{}", cnt)
                        }
                        _ => {}
                    }
                }
            }
            _ => println!("other form of messages!"),
        }
    }
}

async fn send_req<R: RemoteCall>(
    caller: &mut Caller,
    request: &R,
) -> Result<R::Return, InvocationError> {
    caller.client.invoke(request).await
}

async fn send_req_dep<R: RemoteCall>(
    g: &types::G,
    request: &R,
) -> Result<R::Return, InvocationError> {
    let mut m = g.clients.lock().unwrap();

    let mut s = m
        .get_mut()
        .get_session()
        .await
        .unwrap()
        .lock()
        .unwrap()
        .invoke(request)
        .await;
    s
}
