use async_std::task;
use grammers_client::{AuthorizationError, Client, Config};
use grammers_mtsender::InvocationError;
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};
use grammers_tl_types::RemoteCall;
use std::io::Write;

use crate::{errors::GenErr, types};

pub struct Caller {
    pub client: Client,
}

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
                        ci.megagroup = ch.megagroup;
                        ci.restricted = ch.restricted;
                    }
                    _ => {}
                };
                println!("channel info {:#?}", ci);
                return Ok(ci);
            }
        }
    }

    Err(GenErr::TGConverter)
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
    Err(GenErr::TGConverter)
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
    println!("messages #{:#?}", mt);
    process_channel_msgs(caller, mt).await
}

pub async fn get_file(caller: &mut Caller, req: tl::types::InputFileLocation) {
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::Location(req),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await.unwrap();
    // println!("get_chat_id:  {:#?}", res);
}

pub async fn get_file_photo(caller: &mut Caller, req: tl::types::InputPhotoFileLocation) {
    // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  {:#?}", req);
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::InputPhotoFileLocation(req.clone()),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await.unwrap();

    std::fs::create_dir_all("./out/").unwrap();
    let name = format!("./out/{}.jpg", req.id);
    let mut f = std::fs::File::create(name).unwrap();

    use tl::enums::upload::File;

    match res {
        File::File(tfile) => {
            f.write(&tfile.bytes);
        }
        File::CdnRedirect(red) => {}
    };
}

pub async fn get_file_doc(caller: &mut Caller, req: tl::types::InputDocumentFileLocation) {
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
        let res = send_req(caller, &request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        break;
                    }
                };
            }
            Err(err) => {
                break;
            }
        }
        //println!("%%%%%% get_file_photo :  {:#?}", res);
    }

    if out_buffer.len() == 0 {
        return;
    }

    std::fs::create_dir_all("./out/").unwrap();
    let name = format!("./out/{}.file", req.id);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&out_buffer);
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
            msg_holder.channels = process_inline_channel_chats(cm.chats.clone());
            process_inline_channel_users(&cm.users);
            let res = process_inline_channel_messages(cm.messages.clone());
            msg_holder.msgs = res.0;
            msg_holder.urls = res.1;
        }
        _ => println!("other form of messages!"),
    };
    Ok(msg_holder)
    // println!("msgs {:#?} ", msgs);
    // println!("urls {:#?} ", urls);
}

fn process_inline_channel_messages(
    messages: Vec<tl::enums::Message>,
) -> (Vec<types::Msg>, Vec<String>) {
    let mut msgs = vec![];
    let mut urls: Vec<String> = vec![];

    for msg_enum in messages {
        match msg_enum {
            Message::Empty(em) => {}
            Message::Service(service_msg) => {}
            Message::Message(m) => {
                if m.fwd_from.is_some() {
                    // println!(">>> msg fwd \n {:#?}", m2);
                }

                let mut ms = message_to_msg(m.clone());
                let mut u = extract_urls_from_message_entity(m.entities);

                if let Some(f) = m.media.clone() {
                    ms.media = Some(process_inline_media(f));
                    // println!(">>>> file meida {:#?}", f);
                    /*use tl::enums::MessageMedia;
                    match f {
                        MessageMedia::Photo(photo) => {
                            if let Some(pic) = photo.photo {
                                use tl::enums::Photo;
                                match pic {
                                    Photo::Photo(photo) => {
                                        let p = photo;
                                        let inp = tl::types::InputPhotoFileLocation {
                                            id: p.id,
                                            access_hash: p.access_hash,
                                            file_reference: p.file_reference,
                                            thumb_size: "w".to_string(),
                                        };
                                        // get_file_photo(caller, inp).await;
                                    }
                                    _ => {}
                                }
                            }
                        }

                        MessageMedia::Document(doc) => {
                            println!("============== document {:#?}", doc);
                            if let Some(document) = doc.document {
                                use tl::enums::Document;
                                match document {
                                    Document::Document(doc) => {
                                        let d = doc;
                                        let f = tl::types::InputDocumentFileLocation {
                                            id: d.id,
                                            access_hash: d.access_hash,
                                            file_reference: d.file_reference,
                                            thumb_size: "w".to_string(),
                                        };
                                        // get_file_doc(caller, f).await;
                                    },
                                    Document::Empty(e) => {},
                                }
                            }
                        },
                        MessageMedia::Empty(t) => {},
                        MessageMedia::Geo(t) => {},
                        MessageMedia::Contact(t) => {},
                        MessageMedia::Unsupported(t) => {},
                        MessageMedia::WebPage(t) => {
                            println!("********** webpage {:#?}", t);
                        },
                        MessageMedia::Venue(t) => {},
                        MessageMedia::Game(t) => {},
                        MessageMedia::Invoice(t) => {},
                        MessageMedia::GeoLive(t) => {},
                        MessageMedia::Poll(t) => {},
                    }*/
                }

                urls.append(&mut u);
                msgs.push(ms);
            }
        }
    }

    (msgs, urls)
}

fn process_inline_channel_chats(chats: Vec<tl::enums::Chat>) -> Vec<types::ChannelInfo> {
    let mut out = vec![];

    for chat in chats {
        let mut ci = types::ChannelInfo::default();

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
                ci.megagroup = ch.megagroup;
                ci.restricted = ch.restricted;

                out.push(ci);
            }
            _ => {}
        };
    }
    out
}

fn process_inline_channel_users(bots: &Vec<tl::enums::User>) {}

fn process_inline_media(mm: tl::enums::MessageMedia) -> types::Media {
    let mut m = types::Media::default();

    use types::MediaType;
    use tl::enums::MessageMedia;
    match mm {
        MessageMedia::Photo(photo) => {
            m.media_type = MediaType::Image;
            m.ttl_seconds = photo.ttl_seconds.unwrap_or(0);
            if let Some(pic) = photo.photo {
                use tl::enums::Photo;
                match pic {
                    Photo::Photo(photo) => {
                        let p = photo;
                        m.has_sticker = p.has_stickers;
                        m.id = p.id;
                        m.access_hash = p.access_hash;
                        m.file_reference = p.file_reference;
                        m.dc_id = p.dc_id;

                        for s in p.sizes {
                            use tl::enums::PhotoSize;
                            match s {
                                PhotoSize::Size(ps) => {
                                    if m.size < ps.size {
                                        // select the maximum one
                                        m.w = ps.w;
                                        m.h = ps.h;
                                        m.size = ps.size;

                                        let fl = conv_file_location(ps.location);
                                        m.deprecated_volume_id = fl.0;
                                        m.deprecated_local_id = fl.1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        /*let inp = tl::types::InputPhotoFileLocation {
                            id: p.id,
                            access_hash: p.access_hash,
                            file_reference: p.file_reference,
                            thumb_size: "w".to_string(),
                        };*/
                        // get_file_photo(caller, inp).await;
                    }
                    Photo::Empty(e) => {}
                }
            }
        }

        MessageMedia::Document(doc) => {
            println!("============== document {:#?}", doc);
            m.ttl_seconds = doc.ttl_seconds.unwrap_or(0);
            if let Some(document) = doc.document {
                use tl::enums::Document;
                match document {
                    Document::Document(doc) => {
                        let p = doc;
                        m.media_type = MediaType::File;

                        m.id = p.id;
                        m.access_hash = p.access_hash;
                        m.file_reference = p.file_reference;
                        m.date = p.date;
                        m.mime_type = p.mime_type;
                        m.size = p.size;
                        m.dc_id = p.dc_id;

                        for atr in p.attributes {
                            use tl::enums::DocumentAttribute;
                            match atr {
                                DocumentAttribute::ImageSize(s) => {
                                    m.media_type = MediaType::File;
                                    m.w = s.w;
                                    m.h = s.h;
                                }
                                DocumentAttribute::Animated(s) => {
                                    m.animated = true;
                                }
                                DocumentAttribute::Sticker(s) => {}
                                DocumentAttribute::Video(s) => {
                                    m.media_type = MediaType::Video;
                                    m.round_message = s.round_message;
                                    m.supports_streaming = s.supports_streaming;
                                    m.duration = s.duration;
                                    m.w = s.w;
                                    m.h = s.h;
                                }
                                DocumentAttribute::Audio(s) => {
                                    m.media_type = MediaType::Audio;
                                    m.voice = s.voice;
                                    m.duration = s.duration;
                                    m.title = s.title.unwrap_or("".to_string());
                                    m.performer = s.performer.unwrap_or("".to_string());
                                    m.waveform = s.waveform.unwrap_or(vec![]);
                                }
                                DocumentAttribute::Filename(s) => {
                                    m.file_name = s.file_name;
                                }
                                DocumentAttribute::HasStickers(s) => {
                                    m.has_stickers = true;
                                }
                            }
                        }

                        /*let d = doc;
                        let f = tl::types::InputDocumentFileLocation {
                            id: d.id,
                            access_hash: d.access_hash,
                            file_reference: d.file_reference,
                            thumb_size: "w".to_string(),
                        };*/
                        // get_file_doc(caller, f).await;
                    }
                    Document::Empty(e) => {}
                }
            }
        }
        MessageMedia::Empty(t) => {}
        MessageMedia::Geo(t) => {}
        MessageMedia::Contact(t) => {}
        MessageMedia::Unsupported(t) => {}
        MessageMedia::WebPage(t) => {
            println!("********** webpage {:#?}", t);
        }
        MessageMedia::Venue(t) => {}
        MessageMedia::Game(t) => {}
        MessageMedia::Invoice(t) => {}
        MessageMedia::GeoLive(t) => {}
        MessageMedia::Poll(t) => {}
    };

    m
}

fn message_to_msg(m: tl::types::Message) -> types::Msg {
    let mut fwd = None;
    if let Some(fw) = m.fwd_from {
        use tl::enums::MessageFwdHeader::*;
        match fw {
            Header(f) => {
                fwd = Some(types::MsgForwarded {
                    from_id: f.from_id.unwrap_or(0),
                    from_name: f.from_name.unwrap_or("".to_string()),
                    date: f.date,
                    channel_id: f.channel_id.unwrap_or(0),
                    channel_post: f.channel_post.unwrap_or(0),
                    post_author: f.post_author.unwrap_or("".to_string()),
                    saved_from_msg_id: f.saved_from_msg_id.unwrap_or(0),
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
        replay: None,
        media: None,
    }
}

fn extract_urls_from_message_entity(
    entities: Option<Vec<tl::enums::MessageEntity>>,
) -> Vec<String> {
    let mut urls = vec![];
    if let Some(ent) = entities {
        for v in ent {
            use tl::enums::MessageEntity::*;
            match v {
                TextUrl(t) => urls.push(t.url),
                _ => {}
            }
        }
    };
    urls
}

fn conv_file_location(fl: tl::enums::FileLocation) -> (i64, i32) {
    match fl {
        tl::enums::FileLocation::ToBeDeprecated(l) => (l.volume_id, l.local_id),
    }
}
////////////////////////////////////// Archives ////////////////////////////////////

pub async fn get_contacts(g: &types::G) {
    let request = tl::functions::contacts::GetContacts { hash: 23 };
    let mt: tl::enums::contacts::Contacts = send_req_dep(g, &request).await.unwrap();
    // println!("contacts {:#?}", mt);
}

pub async fn get_dialogs(g: &types::G) {
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

pub async fn get_chat_id(g: &types::G) {
    let request = tl::functions::contacts::GetContactIds { hash: 1149267300 };
    let res = send_req_dep(g, &request).await.unwrap();
    // println!("get_chat_id:  {:#?}", res);
}

pub async fn bench_messages_loading_flood(g: &types::G) {
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

//////////////////////////////////// Temp bk ////////////////////////////////////////

/*
async fn process_channel_msgs_with_all_files_(
    caller: &mut Caller,
    mt: tl::enums::messages::Messages,
) -> Result<Vec<types::Msg>, GenErr> {
    let mut msgs = vec![];
    let mut urls: Vec<String> = vec![];
    match mt {
        Messages::ChannelMessages(cm) => {
            process_inline_channel_chats(&cm.chats);
            process_inline_channel_users(&cm.users);
            process_inline_channel_messages(cm.messages.clone());

            for m in cm.messages {
                match m {
                    Message::Message(m2) => {
                        if m2.fwd_from.is_some() {
                            // println!(">>> msg fwd \n {:#?}", m2);
                        }
                        if let Some(f) = m2.media.clone() {
                            // println!(">>>> file meida {:#?}", f);
                            use tl::enums::MessageMedia;
                            match f {
                                MessageMedia::Photo(photo) => {
                                    if let Some(pic) = photo.photo {
                                        use tl::enums::Photo;
                                        match pic {
                                            Photo::Photo(photo) => {
                                                let p = photo;
                                                let inp = tl::types::InputPhotoFileLocation {
                                                    id: p.id,
                                                    access_hash: p.access_hash,
                                                    file_reference: p.file_reference,
                                                    thumb_size: "w".to_string(),
                                                };
                                                get_file_photo(caller, inp).await;
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                MessageMedia::Document(doc) => {
                                    if let Some(document) = doc.document {
                                        use tl::enums::Document;
                                        match document {
                                            Document::Document(doc) => {
                                                let d = doc;
                                                let f = tl::types::InputDocumentFileLocation {
                                                    id: d.id,
                                                    access_hash: d.access_hash,
                                                    file_reference: d.file_reference,
                                                    thumb_size: "w".to_string(),
                                                };
                                                get_file_doc(caller, f).await;
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }

                        let ms = message_to_msg(m2.clone());
                        let mut u = extract_urls_from_message_entity(m2.entities);
                        urls.append(&mut u);
                        msgs.push(ms);
                    },
                    Message::Service(service_msg) => {},
                    Message::Empty(em) => {}
                }
            }
        }
        _ => println!("other form of messages!"),
    };
    Ok(msgs)
    // println!("msgs {:#?} ", msgs);
    // println!("urls {:#?} ", urls);
}

pub async fn get_messages_bk(
    caller: &mut Caller,
    req: ReqGetMessages,
) -> Result<Vec<types::Msg>, GenErr> {
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
    println!("messages #{:#?}", mt);
    process_channel_msgs(caller, mt).await
}




 */
