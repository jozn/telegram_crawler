use std::collections::HashMap;
use std::borrow::Borrow;
use grammers_tl_types::Serializable;
use grammers_tl_types::enums::contacts::ResolvedPeer;
use std::io::Write;
use grammers_client::{AuthorizationError, Client, Config};
// use futures::AsyncWriteExt;
use std::sync::{Arc,Mutex};
use std::cell::Cell;

// pub type G = Arc<Mutex<App>>;
pub type G = Arc<App>;

use crate::client_pool;

pub struct App {
    pub login: Vec<LoginPhone>,
    pub channels: HashMap<i64,ChannelSpace>,
    pub sessions: Vec<Session>,
    pub dcs: Vec<DC>,
    // pub client: Client,
    pub clients: Mutex<Cell<client_pool::ClientPool>>,
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
    pub info: ChannelInfo,
    pub msgs: HashMap<u32,Msg>,

}

#[derive(Clone, Default, Debug)]
pub struct ChannelInfo {
    pub id: i32,
    pub title: String,
    pub username: String,
    pub about: String,
    pub link: String,
    pub members_count: i32,
    pub read_inbox_max_id: i32,
    pub access_hash: i64,
    pub date: i32,
    pub photo: u8,
    pub version: i32,
    pub pts: i32,
    pub restricted: bool,
    pub megagroup: bool,
}

#[derive(Clone, Default, Debug)]
pub struct ChannelByUsernameResult {
    pub id: i32,
    pub title: String,
    pub username: String,
    pub access_hash: i64,
    pub date: i32,
    pub photo: u8,
    pub version: i32,
    pub restricted: bool,
    pub megagroup: bool,
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