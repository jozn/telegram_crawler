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
    // let mut app = Arc::new(Mutex::new(app));
    // let app1 = app.get_mut().unwrap();

    // app.get_dialogs().await;
    app.get_messages().await;
    // app.get_channel_info().await;
    // app.get_channel_by_username().await;
    // app.get_chat_id().await;

}

// types
use std::collections::HashMap;
use std::borrow::Borrow;
use grammers_tl_types::Serializable;
use grammers_tl_types::enums::contacts::ResolvedPeer;
use std::io::Write;
// use futures::AsyncWriteExt;

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
    pub from_id: i32,
    pub from_name: String,
    pub date: i32,
    pub channel_id: i32,
    pub channel_post: i32,
    pub post_author: String,
    // pub saved_from_peer: Option<crate::enums::Peer>,
    pub saved_from_msg_id: i32,
}

#[derive(Clone, Debug)]
pub struct Msg {
    pub silent: bool,
    pub post: bool,
    pub id: i32,
    pub from_id: i32,
    // pub to_id: crate::enums::Peer,
    // pub fwd_from: Option<crate::enums::MessageFwdHeader>,
    pub via_bot_id: i32,
    pub reply_to_msg_id: i32,
    pub date: i32,
    pub message: String,
    // pub media: Option<crate::enums::MessageMedia>,
    // pub reply_markup: Option<crate::enums::ReplyMarkup>,
    // pub entities: Option<Vec<crate::enums::MessageEntity>>,
    pub views: i32,
    pub edit_date: i32,
    // pub post_author: Option<String>,
    // pub grouped_id: Option<i64>,
    pub restricted: bool,
    pub forward: Option<MsgForwarded>,
    pub replay: Option<MsgReplayTo>,
    // raw: tl::types::Message,
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

#[derive(Clone, Default, Debug)]
pub struct ChannelInfo {
    id: i32,
    title: String,
    username: String,
    about: String,
    link: String,
    members_count: i32,
    read_inbox_max_id: i32,
    access_hash: i64,
    date: i32,
    photo: u8,
    version: i32,
    pts: i32,
    restricted: bool,
    megagroup: bool,
}

#[derive(Clone, Default, Debug)]
pub struct ChannelByUsernameResult {
    id: i32,
    title: String,
    username: String,
    access_hash: i64,
    date: i32,
    photo: u8,
    version: i32,
    restricted: bool,
    megagroup: bool,
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
    pub async fn get_contacts(&mut self) {
        // get contacts
        let request = tl::functions::contacts::GetContacts {
            hash:23,
        };
        let mt : tl::enums::contacts::Contacts = self.client.invoke(&request).await.unwrap();
        // println!("contacts {:#?}", mt);
    }

    pub async fn get_dialogs(&mut self) {
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
        let mt : tl::enums::messages::Dialogs = self.client.invoke(&request).await.unwrap();
        println!("dilagos {:#?}", mt);
    }

    pub async fn get_channel_info(&mut self) {
        let request = tl::functions::channels::GetFullChannel {
            channel: tl::enums::InputChannel::Channel(
                tl::types::InputChannel{
                    channel_id: 1072723547,
                    access_hash: -1615658883512673699,
                }
            )
        };
        // let res: tl::enums::ChatFull = self.client.invoke(&request).await.unwrap();
        let res = self.client.invoke(&request).await.unwrap();
        println!("request {:#?}", res);

        let mut ci = ChannelInfo::default();

        use tl::enums::messages::ChatFull;
        match res {
            ChatFull::Full(full)=> {

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

                if full.chats.len() == 1{
                    let chat = full.chats.get(0).unwrap();

                    use tl::enums::Chat;
                    match chat {
                        Chat::Channel(ch) => {
                            ci.id = ch.id;
                            ci.title = ch.title.clone();
                            ci.username = ch.username.borrow().as_ref().unwrap_or(&"".to_string()).clone();
                            ci.access_hash = ch.access_hash.unwrap_or(0);
                            ci.date =  ch.date;
                            ci.version =  ch.version;
                            // ci.members_count = ch.participants_count.unwrap_or(0); // Note: it is None in here! use 'full_chat'
                            ci.megagroup = ch.megagroup;
                            ci.restricted = ch.restricted;
                        }
                        _ => {}
                    }
                }
            }
        }

        println!("channel info {:#?}", ci);

    }

    pub async fn get_channel_by_username(&mut self) {
        let request = tl::functions::contacts::ResolveUsername {
            // username: "Arsshiy_Fortnite".to_string(),
            // username: "badansazizanan".to_string(),
            // username: "arzansaraereza".to_string(),
            username: "pornstar_15".to_string(),
        };
        // let res: tl::enums::ChatFull = self.client.invoke(&request).await.unwrap();
        let res = self.client.invoke(&request).await.unwrap();
        println!("resolve username:  {:#?}", res);

        use tl::enums::contacts::ResolvedPeer;
        match res {
            ResolvedPeer::Peer(peer) => {

                use tl::enums::Chat;
                for chat in peer.chats {
                    match chat {
                        Chat::Channel(channel) => {
                            let c = channel;
                            let res = ChannelByUsernameResult {
                                id: c.id,
                                title: c.title,
                                username: c.username.unwrap_or("".to_string()),
                                access_hash: c.access_hash.unwrap_or(0),
                                date: c.date,
                                photo: 0,
                                version: c.version,
                                restricted: c.restricted,
                                megagroup: c.megagroup
                            };
                            println!(">>> channel: #{:#?} ", res);
                        },
                        _  => {

                        }
                    }
                }
            }
        }
    }

    pub async fn get_chat_id(&mut self) {
        let request = tl::functions::contacts::GetContactIds {
            hash:1149267300,
        };
        // let res: tl::enums::ChatFull = self.client.invoke(&request).await.unwrap();
        let res = self.client.invoke(&request).await.unwrap();
        println!("get_chat_id:  {:#?}", res);

    }

    pub async fn get_messages(&mut self) {
        let request = tl::functions::messages::GetHistory {
            /*peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1072723547, // telegraph
                access_hash: -1615658883512673699,
            }),*/

            /*peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1355843251, // codal365
                access_hash: -6028453276089081451,
            }),*/
            peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1163672339, // forever54321
                access_hash: -3665401744061121093,
            }),
           /* peer: tl::enums::InputPeer::Channel(tl::types::InputPeerChannel{
                channel_id: 1220769397, // forever54321
                access_hash: -6783224835856251633,
            }),*/
            offset_id: 0,
            offset_date: 0,
            add_offset: 0,
            limit: 100,
            max_id: 0,
            min_id: 0,
            hash: 0
        };

        let mt : tl::enums::messages::Messages = self.client.invoke(&request).await.unwrap();
        // println!("messages #{:#?}", mt);
        // process_msgs(mt, &mut self);
        self.process_msgs(mt).await;
    }

    pub async fn get_file(&mut self, req : tl::types::InputFileLocation ) {
        // let fl = tl::enums::InputFileLocation;
        let request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::Location(req),
            offset: 0,
            limit: 524288
        };
        let res = self.client.invoke(&request).await.unwrap();
        println!("get_chat_id:  {:#?}", res);

    }

    pub async fn get_file_photo(&mut self, req : tl::types::InputPhotoFileLocation ) {
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  {:#?}", req);
        let request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputPhotoFileLocation(req.clone()),
            offset: 0,
            limit: 524288
        };
        let res = self.client.invoke(&request).await.unwrap();
        println!("%%%%%% get_file_photo :  {:#?}", res);

        std::fs::create_dir_all("./out/").unwrap();
        let name = format!("./out/{}.jpg", req.id);
        let mut f = std::fs::File::create(name).unwrap();

        use tl::enums::upload::File;

        match res {
            File::File(tfile) => {
                f.write(&tfile.bytes);
            },
            File::CdnRedirect(red) => {

            },
        };
    }

    pub async fn get_file_doc(&mut self, req : tl::types::InputDocumentFileLocation ) {
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  {:#?}", req);
        let limit = 524288;
        let mut out_buffer = Vec::with_capacity(limit as usize);
        let mut offset = 0;

        loop {
            let request = tl::functions::upload::GetFile {
                precise: false,
                cdn_supported: false,
                location: tl::enums::InputFileLocation::InputDocumentFileLocation(req.clone()),
                offset: offset,
                limit: limit,
            };
            let res = self.client.invoke(&request).await;

            match res {
                Ok(res) => {
                    use tl::enums::upload::File;
                    match res {
                        File::File(tfile) => {
                            let len = tfile.bytes.len() as i32;
                            out_buffer.write(&tfile.bytes);
                            if len ==  limit {
                                  offset = offset + limit;
                            } else {
                                break;
                            }
                        },
                        File::CdnRedirect(red) => {
                            break;
                        },
                    };
                },
                Err(err) => {
                   break;
                }
            }
            //println!("%%%%%% get_file_photo :  {:#?}", res);
        }

        if out_buffer.len() == 0 {
            return
        }

        std::fs::create_dir_all("./out/").unwrap();
        let name = format!("./out/{}.file", req.id);
        let mut f = std::fs::File::create(name).unwrap();
        f.write(&out_buffer);

        // use tl::enums::upload::File;

        /*match res {
            File::File(tfile) => {
                f.write(&tfile.bytes);
            },
            File::CdnRedirect(red) => {

            },
        };*/
    }

    pub async fn bench_messages_loading_flood(&mut self) {
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

        let mt : tl::enums::messages::Messages = self.client.invoke(&request).await.unwrap();

        let mut cnt = 0;
        for i in 1..500 {
            println!("> {} -- ", i);
            let mt : tl::enums::messages::Messages = self.client.invoke(&request).await.unwrap();

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

    async fn process_msgs(&mut self, mt: tl::enums::messages::Messages) {
        let mut msgs = vec![];
        let mut urls :Vec<String> = vec![];
        match mt {
            Messages::ChannelMessages(m) => {
                for m in m.messages {
                    match m {
                        Message::Message(m2) => {

                            if m2.fwd_from.is_some() {
                                // println!(">>> msg fwd \n {:#?}", m2);
                            }
                            if let Some(f) = m2.media.clone() {
                                println!(">>>> file meida {:#?}", f);
                                use tl::enums::MessageMedia;
                                match f {
                                    MessageMedia::Photo(photo) => {
                                        if let Some(pic) = photo.photo {
                                            use tl::enums::Photo;
                                            match pic {
                                                Photo::Photo(photo) => {
                                                    let p = photo;
                                                    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ");
                                                    /*let g=  tl::types::InputFileLocation{
                                                        volume_id: 0,
                                                        local_id: 0,
                                                        secret: p.access_hash,
                                                        file_reference: p.file_reference,
                                                    };
                                                    p.sizes*/
                                                    let g=  tl::types::InputPhotoFileLocation{
                                                        id: p.id,
                                                        access_hash: p.access_hash,
                                                        file_reference: p.file_reference,
                                                        thumb_size: "w".to_string()
                                                    };
                                                    self.get_file_photo(g).await;
                                                }
                                                _ => {}
                                            }
                                        }
                                    },

                                    MessageMedia::Document(doc) => {
                                        if let Some(document) = doc.document {
                                            use tl::enums::Document;
                                            match document {
                                                Document::Document(doc) => {
                                                    let d = doc;
                                                    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ ");
                                                    /*let g=  tl::types::InputFileLocation{
                                                        volume_id: 0,
                                                        local_id: 0,
                                                        secret: p.access_hash,
                                                        file_reference: p.file_reference,
                                                    };
                                                    p.sizes*/
                                                    let g=  tl::types::InputDocumentFileLocation{
                                                        id: d.id,
                                                        access_hash: d.access_hash,
                                                        file_reference: d.file_reference,
                                                        thumb_size: "w".to_string()
                                                    };
                                                    self.get_file_doc(g).await;
                                                }
                                                _ => {}
                                            }
                                        }
                                    },


                                    _ => {}
                                }

                                // app.get_file();
                            }

                            let ms = message_to_msg(m2.clone());
                            let mut u =  extract_urls_from_message_entity(m2.entities);
                            urls.append(&mut u);
                            msgs.push(ms);
                        },
                        _ => {}
                    }
                }
            },
            _ => {
                println!("other form of messages!")
            }
        }
        // println!("msgs {:#?} ", msgs);
        // println!("urls {:#?} ", urls);
    }

}

struct getFileReq {

}

fn process_msgs(mt: tl::enums::messages::Messages, app: &mut App) {
    let mut msgs = vec![];
    let mut urls :Vec<String> = vec![];
    match mt {
        Messages::ChannelMessages(m) => {
            for m in m.messages {
                match m {
                    Message::Message(m2) => {

                        if m2.fwd_from.is_some() {
                            // println!(">>> msg fwd \n {:#?}", m2);
                        }
                        if let Some(f) = m2.media.clone() {
                            println!(">>>> file meida {:?}", f)

                            // app.get_file();
                        }

                        let ms = message_to_msg(m2.clone());
                        let mut u =  extract_urls_from_message_entity(m2.entities);
                        urls.append(&mut u);
                        msgs.push(ms);
                    },
                    _ => {}
                }
            }
        },
        _ => {
            println!("other form of messages!")
        }
    }
    // println!("msgs {:#?} ", msgs);
    // println!("urls {:#?} ", urls);
}

fn message_to_msg(m: tl::types::Message) -> Msg{
    let mut fwd = None ;
    if let Some(fw) = m.fwd_from {
        use tl::enums::MessageFwdHeader::*;
        match fw {
            Header (f) => {
                fwd = Some(MsgForwarded {
                    from_id: f.from_id.unwrap_or(0),
                    from_name: f.from_name.unwrap_or("".to_string()),
                    date: f.date,
                    channel_id: f.channel_id.unwrap_or(0),
                    channel_post: f.channel_post.unwrap_or(0),
                    post_author: f.post_author.unwrap_or("".to_string()),
                    saved_from_msg_id: f.saved_from_msg_id.unwrap_or(0)
                });
            }


        }
    };
    // println!("forward {:#?} ", fwd);
    Msg {
        silent: m.silent,
        post: m.post,
        id: m.id,
        from_id: m.id,
        via_bot_id: m.via_bot_id.unwrap_or(0),
        reply_to_msg_id: m.reply_to_msg_id.unwrap_or(0),
        date: m.date,
        message: m.message,
        views: m.views.unwrap_or(0),
        edit_date: m.edit_date.unwrap_or(0),
        restricted: m.restriction_reason.is_some(),
        forward: fwd,
        replay: None
    }
}

fn extract_urls_from_message_entity(entities: Option<Vec<tl::enums::MessageEntity>>) -> Vec<String>{
    let mut urls = vec![];
    if let Some(ent) = entities {
        for v in ent {
            use tl::enums::MessageEntity::*;
            match v {
                TextUrl(t)=> {
                    urls.push(t.url)
                },
                _ => {},
            }
        }
    };
    urls
}
