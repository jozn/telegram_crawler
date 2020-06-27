
use async_std::task;
use grammers_client::{AuthorizationError, Client, Config};
use grammers_tl_types as tl;
use grammers_session as session;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};
use grammers_tl_types::RemoteCall;
use grammers_mtsender::InvocationError;
use std::io::Write;

use crate::types;

async fn send_req<R: RemoteCall>(g: &types::G, request: &R) -> Result<R::Return,InvocationError>{
    let mut m = g.clients.lock().unwrap();

    let mut s = m.get_mut().get_session().await.unwrap().lock().unwrap().invoke(request).await;
    s
}
pub async fn get_contacts(g :&mut types::G) {
    // get contacts
    let request = tl::functions::contacts::GetContacts {
        hash:23,
    };
    let mt : tl::enums::contacts::Contacts = g.get_mut().client.invoke(&request).await.unwrap();
    // println!("contacts {:#?}", mt);
}

pub async fn get_dialogs(g :&mut types::G) {
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
    let mt : tl::enums::messages::Dialogs = g.get_mut().client.invoke(&request).await.unwrap();
    println!("dilagos {:#?}", mt);
}

pub async fn get_channel_info(g :&mut types::G) {
    let request = tl::functions::channels::GetFullChannel {
        channel: tl::enums::InputChannel::Channel(
            tl::types::InputChannel{
                channel_id: 1072723547,
                access_hash: -1615658883512673699,
            }
        )
    };
    // let res: tl::enums::ChatFull = self.client.invoke(&request).await.unwrap();
    let res = g.client.invoke(&request).await.unwrap();
    println!("request {:#?}", res);

    let mut ci = types::ChannelInfo::default();

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
                        ci.username = ch.username.clone().as_ref().unwrap_or(&"".to_string()).clone();
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

pub async fn get_channel_by_username(g :&mut types::G) {
    let request = tl::functions::contacts::ResolveUsername {
        // username: "Arsshiy_Fortnite".to_string(),
        // username: "badansazizanan".to_string(),
        // username: "arzansaraereza".to_string(),
        username: "pornstar_15".to_string(),
    };
    // let res: tl::enums::ChatFull = self.client.invoke(&request).await.unwrap();
    let res = g.client.invoke(&request).await.unwrap();
    println!("resolve username:  {:#?}", res);

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

pub async fn get_chat_id(g :&mut types::G) {
    let request = tl::functions::contacts::GetContactIds {
        hash:1149267300,
    };
    // let res: tl::enums::ChatFull = self.client.invoke(&request).await.unwrap();
    let res = g.client.invoke(&request).await.unwrap();
    println!("get_chat_id:  {:#?}", res);

}

pub async fn get_messages(g :&mut types::G) {
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

    let mt : tl::enums::messages::Messages = g.client.invoke(&request).await.unwrap();
    // println!("messages #{:#?}", mt);
    // process_msgs(mt, g :&mut types::G);
    process_msgs(g,mt).await;
}

pub async fn get_file(g :&mut types::G, req : tl::types::InputFileLocation ) {
    // let fl = tl::enums::InputFileLocation;
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::Location(req),
        offset: 0,
        limit: 524288
    };
    let res = g.client.invoke(&request).await.unwrap();
    println!("get_chat_id:  {:#?}", res);

}

pub async fn get_file_photo(g :&mut types::G, req : tl::types::InputPhotoFileLocation ) {
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  {:#?}", req);
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::InputPhotoFileLocation(req.clone()),
        offset: 0,
        limit: 524288
    };
    let res = g.client.invoke(&request).await.unwrap();
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

pub async fn get_file_doc(g :&mut types::G, req : tl::types::InputDocumentFileLocation ) {
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
        let res = g.client.invoke(&request).await;

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

pub async fn bench_messages_loading_flood(g :&mut types::G) {
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

    let mt : tl::enums::messages::Messages = g.client.invoke(&request).await.unwrap();

    let mut cnt = 0;
    for i in 1..500 {
        println!("> {} -- ", i);
        let mt : tl::enums::messages::Messages = g.client.invoke(&request).await.unwrap();

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

async fn process_msgs(g :&mut types::G, mt: tl::enums::messages::Messages) {
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
                                                get_file_photo(mut g.clone()).await;
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
                                                g.get_file_doc(g).await;
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



    fn process_msgs22(mt: tl::enums::messages::Messages) {
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

    fn message_to_msg(m: tl::types::Message) -> types::Msg{
        let mut fwd = None ;
        if let Some(fw) = m.fwd_from {
            use tl::enums::MessageFwdHeader::*;
            match fw {
                Header (f) => {
                    fwd = Some(types::MsgForwarded {
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
        types::Msg {
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