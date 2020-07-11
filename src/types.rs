use grammers_client::{AuthorizationError, Client, Config};
use grammers_tl_types::enums::contacts::ResolvedPeer;
use grammers_tl_types::Serializable;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Write;
// use futures::AsyncWriteExt;
use grammers_tl_types as tl;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::fmt;

// pub type G = Arc<Mutex<App>>;
pub type G = Arc<App>;
pub type Binary = Vec<u8>;

use crate::client_pool;
use serde::{Deserialize, Serialize};

pub struct App {
    pub login: Vec<LoginPhone>,
    pub channels: HashMap<i64, ChannelSpace>,
    pub sessions: Vec<Session>,
    pub dcs: Vec<DC>,
    // pub client: Client,
    pub clients: Mutex<Cell<client_pool::ClientPool>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MsgReplayTo {}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
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

    pub media: Option<Media>,
    // raw: tl::types::Message,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MediaType {
    Unknown,
    Image,
    Video,
    Audio,
    File,
    ImageFile,
}

// #[derive(Derivative)]
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
// #[derive(Clone, Debug, Default)]
pub struct Media {
    pub media_type: MediaType,
    pub has_stickers: bool,
    pub id: i64,
    pub access_hash: i64,
    // #[derivative(Debug="ignore")]
    pub file_reference: Vec<u8>,
    pub date: i32,
    // pub sizes: Vec<tl::enums::PhotoSize>,
    pub dc_id: i32,

    // FileLocationToBeDeprecated
    pub deprecated_volume_id: i64,
    pub deprecated_local_id: i32,

    // pub location: tl::enums::FileLocation,
    pub w: i32,
    pub h: i32,
    pub size: i32,

    // Document
    // pub id: i64,
    // pub access_hash: i64,
    // pub file_reference: Vec<u8>,
    // pub date: i32,
    pub mime_type: String,
    // pub size: i32,
    // pub thumbs: Option<Vec<tl::enums::PhotoSize>>,
    // pub dc_id: i32,
    // pub attributes: Vec<tl::enums::DocumentAttribute>,
    pub animated: bool,

    // Video
    pub round_message: bool,
    pub supports_streaming: bool,
    pub duration: i32,
    pub video_w: i32,
    pub video_h: i32,

    // Audio
    pub voice: bool,
    pub audio_duration: i32, // merge
    pub title: String,
    pub performer: String,
    pub waveform: Vec<u8>,

    pub file_name: String,

    pub has_sticker: bool,
    pub ttl_seconds: i32,

    // Us
    pub file_extention: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChannelSpace {
    pub info: ChannelInfo,
    pub msgs: HashMap<u32, Msg>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DC {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Session {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginPhone {}
/////////// Sqlite ///////
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CachedUsernameData {
    pub username: String,
    pub channel_id: i32, // we do not care about others: super groups, users,...
    // pub tg_result: Option<ChannelByUsernameResult>,
    pub channel_info: Option<ChannelInfo>,
    pub taken: bool,
    pub last_checked: u32,
}

///////////////////////////
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReqSyncChannel {
    pub channel_id: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResSyncChannel {
    pub channel_info: ChannelInfo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReqSyncMessages {
    pub channel_id: i32,
    pub access_id: i64,
    pub from_message_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResSyncMessages {
    pub req: ReqSyncMessages,
    pub messages: Vec<Msg>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReqResolveUsername {
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResResolveUsername {
    pub channel_id: i32,
    pub access_id: i64,
}

impl Default for MediaType {
    fn default() -> Self {
        MediaType::Unknown
    }
}

// impl fmt::Debug for Binary {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "--snip Vec<u8> --",)
//     }
// }
