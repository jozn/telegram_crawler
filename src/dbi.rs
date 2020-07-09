use crate::types;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::new as new_session;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query;
use cdrs::query::*;
// use sqlite;

pub trait DBI_old {
    fn load_channels() -> Vec<types::ChannelInfo>;
    fn load_seed_usernames() -> Vec<String>;
    fn save_channel();
    fn save_channel_msg();

    fn load_sessions();
    fn save_session();
}

pub trait DB {
    fn load_channels() -> Vec<types::ChannelInfo>;
    fn load_seed_usernames() -> Vec<String>;
    fn save_channel();
    fn save_channel_msg();

    fn load_sessions();
    fn save_session();
}

/*
pub fn play1() {
    let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS test_ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    no_compression.query(create_ks).expect("Keyspace create error");

    let s = "INSERT INTO test_ks.channels
    (id)
VALUES
(1 ); ";

    no_compression.query(s).expect("erro");

    // query::QueryParamsBuilder::new()
}

pub fn sqlite_play() {
    // let c = sqlite::open(":memory:").unwrap();
    let c = sqlite::open("./crawling.sqlite").unwrap();

    c
        .execute(
            "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        ;



}
*/
