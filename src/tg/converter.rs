
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

pub(super) fn process_inline_channel_messages(
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

                let mut msg = conv_message_to_msg(m.clone());
                let mut u = extract_urls_from_message_entity(m.entities);

                if let Some(mm) = m.media.clone() {
                    msg.media = process_inline_media(mm.clone());
                    msg.webpage = process_inline_webpage(mm);
                }

                if let Some(rm) = m.reply_markup {
                    msg.markup_urls = process_inline_markup_urls(rm);
                }

                urls.append(&mut u);
                msgs.push(msg);
            }
        }
    }

    (msgs, urls)
}

pub(super) fn process_inline_channel_chats(chats: Vec<tl::enums::Chat>) -> Vec<types::ChannelInfo> {
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

pub(super) fn process_inline_channel_users(bots: &Vec<tl::enums::User>) {}

fn process_inline_media(mm: tl::enums::MessageMedia) -> Option<types::Media> {
    let mut m = types::Media::default();

    use tl::enums::MessageMedia;
    use types::MediaType;
    match mm {
        MessageMedia::Photo(photo) => {
            if let Some(pic) = photo.photo {
                let mp = conv_photo_to_media(pic);
                if let Some(mut mp) = mp {
                    // mp.media_type = MediaType::Image;
                    mp.ttl_seconds = photo.ttl_seconds.unwrap_or(0);
                    return Some(mp);
                }
            }
            /*if let Some(pic) = photo.photo {
                // println!("====== Photo {:#?}", pic);

                use tl::enums::Photo;
                match pic {
                    Photo::Photo(photo) => {
                        let p = photo;
                        m.has_sticker = p.has_stickers;
                        m.id = p.id;
                        m.access_hash = p.access_hash;
                        m.file_reference = p.file_reference;
                        m.dc_id = p.dc_id;
                        m.file_extention = ".jpg".to_string();

                        for s in p.sizes {
                            use tl::enums::PhotoSize;
                            match s {
                                PhotoSize::Size(ps) => {
                                    if m.size < ps.size {
                                        // select the maximum one
                                        m.w = ps.w;
                                        m.h = ps.h;
                                        m.size = ps.size;
                                        m.photo_size_type = ps.r#type;

                                        let fl = conv_file_location(ps.location);
                                        m.dep_volume_id = fl.0;
                                        m.dep_local_id = fl.1;
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
            };*/
            // return Some(m);
        }

        MessageMedia::Document(doc) => {
            // println!("============== document {:#?}", doc);
            m.ttl_seconds = doc.ttl_seconds.unwrap_or(0);
            if let Some(document) = doc.document {
                use tl::enums::Document;
                match document {
                    Document::Document(doc) => {
                        let p = doc.clone();
                        m.media_type = MediaType::File;

                        m.id = p.id;
                        m.access_hash = p.access_hash;
                        m.file_reference = p.file_reference;
                        m.date = p.date;
                        m.mime_type = p.mime_type.clone();
                        m.size = p.size;
                        m.dc_id = p.dc_id;

                        // m.file_extention = mime_db::extension(&p.mime_type).unwrap_or("").to_string();
                        m.file_extention = utils::get_file_extension_from_mime_type(&p.mime_type);

                        //todo move to just video + remove rec
                        if p.thumbs.is_some() {
                            m.video_thumbs_rec =
                                Box::new(conv_vidoe_thumbs_rec(&m, p.thumbs.clone().unwrap()));
                            m.video_thumbs = conv_vidoe_thumbs(p.thumbs.unwrap());
                            // println!("+++ vidoe: {:#?} ", doc)
                        }

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
            };
            return Some(m);
        }
        MessageMedia::Empty(t) => {}
        MessageMedia::Geo(t) => {}
        MessageMedia::Contact(t) => {}
        MessageMedia::Unsupported(t) => {}
        MessageMedia::WebPage(t) => {
            use tl::enums::WebPage;
            match t.webpage {
                WebPage::Empty(v) => {}
                WebPage::Pending(v) => {}
                WebPage::Page(v) => {}
                WebPage::NotModified(v) => {}
            }
            // println!("********** webpage {:#?}", t);
        }
        MessageMedia::Venue(t) => {}
        MessageMedia::Game(t) => {}
        MessageMedia::Invoice(t) => {}
        MessageMedia::GeoLive(t) => {}
        MessageMedia::Poll(t) => {}
    };
    None
}

fn process_inline_webpage(mm: tl::enums::MessageMedia) -> Option<types::WebPage> {
    use tl::enums::MessageMedia;
    match mm {
        MessageMedia::WebPage(t) => {
            use tl::enums::WebPage;
            match t.webpage {
                WebPage::Empty(v) => {}
                WebPage::Pending(v) => {}
                WebPage::Page(v) => {
                    let mut w = types::WebPage {
                        id: v.id,
                        url: v.url,
                        display_url: v.display_url,
                        hash: v.hash,
                        page_type: v.r#type.unwrap_or("".to_string()),
                        site_name: v.site_name.unwrap_or("".to_string()),
                        title: v.title.unwrap_or("".to_string()),
                        description: v.description.unwrap_or("".to_string()),
                        photo: None,
                    };

                    if v.photo.is_some() {
                        w.photo = conv_photo_to_media(v.photo.unwrap())
                    }

                    return Some(w);
                }
                WebPage::NotModified(v) => {}
            }
        }
        _ => {}
    };
    None
}

fn process_inline_markup_urls(ms: tl::enums::ReplyMarkup) -> Option<Vec<types::MarkupUrl>> {
    let mut arr = vec![];
    use tl::enums::ReplyMarkup;
    match ms {
        ReplyMarkup::ReplyInlineMarkup(im) => {
            let mut m = -1;
            for row in im.rows {
                m += 1;

                use tl::enums::KeyboardButtonRow;
                match row {
                    KeyboardButtonRow::Row(br) => {
                        for bts in br.buttons {
                            use tl::enums::KeyboardButton;
                            match bts {
                                KeyboardButton::Url(u) => {
                                    let r = types::MarkupUrl {
                                        row_id: m,
                                        text: u.text,
                                        url: u.url,
                                    };
                                    arr.push(r);
                                }

                                KeyboardButton::UrlAuth(u) => {
                                    // this is for things like comments bot
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    if arr.len() > 0 {
        return Some(arr);
    }
    None
}

fn conv_message_to_msg(m: tl::types::Message) -> types::Msg {
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
        webpage: None,
        markup_urls: None,
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

fn conv_photo_to_media(photo_enum: tl::enums::Photo) -> Option<types::Media> {
    let mut m = types::Media::default();
    use tl::enums::Photo;
    match photo_enum {
        Photo::Photo(photo) => {
            let p = photo;
            m.media_type = types::MediaType::Image;
            m.has_sticker = p.has_stickers;
            m.id = p.id;
            m.access_hash = p.access_hash;
            m.file_reference = p.file_reference;
            m.date = p.date;
            m.dc_id = p.dc_id;
            m.file_extention = ".jpg".to_string();

            for s in p.sizes {
                use tl::enums::PhotoSize;
                match s {
                    PhotoSize::Size(ps) => {
                        if m.size < ps.size {
                            // select the maximum
                            m.w = ps.w;
                            m.h = ps.h;
                            m.size = ps.size;
                            m.photo_size_type = ps.r#type;

                            let fl = conv_file_location(ps.location);
                            m.dep_volume_id = fl.0;
                            m.dep_local_id = fl.1;
                        }
                    }
                    _ => {}
                }
            }
            return Some(m);
        }
        Photo::Empty(e) => {}
    };
    None
}

fn conv_vidoe_thumbs(vts: Vec<tl::enums::PhotoSize>) -> Option<MediaThumb> {
    if vts.len() == 0 {
        return None;
    }

    let mut m = types::MediaThumb::default();

    for vt in vts {
        use tl::enums::PhotoSize;
        match vt {
            PhotoSize::Size(s) => {
                // select the maximum one
                if m.size < s.size {
                    m.size_type = s.r#type;
                    m.w = s.w;
                    m.h = s.h;
                    m.size = s.size;

                    use tl::enums::FileLocation;
                    match s.location {
                        FileLocation::ToBeDeprecated(l) => {
                            m.dep_volume_id = l.volume_id;
                            m.dep_local_id = l.local_id;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Some(m)
}

fn conv_vidoe_thumbs_rec(medid: &types::Media, vts: Vec<tl::enums::PhotoSize>) -> Option<Media> {
    let mut m = Media::default();
    m.id = medid.id;
    m.access_hash = medid.access_hash;
    m.file_reference = medid.file_reference.clone();
    m.file_extention = "jpg".to_string();

    for vt in vts {
        use tl::enums::PhotoSize;
        match vt {
            PhotoSize::Size(s) => {
                // select the maximum one
                if m.size < s.size {
                    m.photo_size_type = s.r#type;
                    m.w = s.w;
                    m.h = s.h;
                    m.size = s.size;

                    use tl::enums::FileLocation;
                    match s.location {
                        FileLocation::ToBeDeprecated(l) => {
                            m.dep_volume_id = l.volume_id;
                            m.dep_local_id = l.local_id;
                        }
                    }
                };
                return Some(m);
            }
            _ => {}
        }
    }
    None
}
