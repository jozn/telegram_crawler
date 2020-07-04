
use crate::{types};

pub trait DBI {
    fn load_channels() -> Vec<types::ChannelInfo>;
    fn load_seed_usernames() -> Vec<String>;
    fn save_channel();
    fn save_channel_msg();

    fn load_sessions();
    fn save_session();

}

