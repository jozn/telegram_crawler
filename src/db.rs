// use sqlite;
use crate::{errors::GenErr, types, utils};

use rusqlite::{params, Connection, Result, NO_PARAMS};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn save_channel(i: &types::ChannelSpace) {
    let c = Connection::open("./crawling.sqlite").unwrap();
    let q = "insert into channels (id, username, data) values(?1,?2,?3)";
    c.execute(q, params![i.info.id, &i.info.username, &i.info.date])
        .unwrap();
}

pub fn load_all_channels() {}

pub fn save_message() {}

pub fn save_file() {}

pub fn save_queue_username(username: &str) {
    let con = get_conn();
    let mut username = username.trim().to_string();
    if !utils::is_valid_username_pattern(&username) {
        return;
    }

    let q = "insert into queue_username (username) values (?1)";
    con.execute(q, params![username]).unwrap();
}

pub fn get_next_queue_username() -> Result<String, GenErr> {
    let con = get_conn();
    let mut s = "".to_string();

    let q = "SELECT username FROM queue_username ORDER BY RANDOM() LIMIT 1";

    let mut stmt = con.prepare(q)?;
    let m = stmt.query_row(params![], |row| {
        s = row.get(0)?;
        Ok(s)
    })?;

    Ok(m)
}

pub fn delete_queue_username(username: &str) {
    let con = get_conn();
    let q = "delete from queue_username where username = ?1";
    con.execute(q, params![username]);
}

pub fn save_cached_username(cud: &types::CachedUsernameData) {
    let con = get_conn();
    let bin = serde_json::to_string(cud).unwrap();
    let q = "REPLACE into cached_username (username, channel_id ,data) values (?1,?2,?3)";
    con.execute(q, params![&cud.username, cud.channel_id, bin])
        .unwrap();
}

pub fn load_all_cached_usernames() {}

pub fn get_random_cached_channel() -> Result<types::CachedUsernameData, GenErr> {
    let con = get_conn();

    // let q = "SELECT data FROM cached_username where channel_id != 0 ORDER BY RANDOM() LIMIT 1";
    let q =
        "SELECT data FROM cached_username where channel_id == 1163672339 ORDER BY RANDOM() LIMIT 1";

    let mut stmt = con.prepare(q)?;
    let m = stmt.query_row(params![], |row| {
        let s: String = row.get(0)?;
        Ok(s)
    })?;
    let ch_info = serde_json::from_str(&m)?;
    Ok(ch_info)
}

// todo: make it lazy static
fn get_conn() -> Connection {
    let con = Connection::open("./crawling.sqlite").unwrap();
    con.execute("PRAGMA synchronous = OFF", params![]);
    con.execute("PRAGMA journal_mode = MEMORY", params![]);
    con.execute("PRAGMA temp_store = MEMORY", params![]);
    con
}

///////////////// Archives ////////////////

pub fn get_next_queue_username_bk() -> Result<String, GenErr> {
    let con = get_conn();
    let mut s = "".to_string();

    let q = "SELECT username FROM queue_username ORDER BY RANDOM() LIMIT 1";

    let mut stmt = con.prepare(q)?;
    let mut rows = stmt.query(NO_PARAMS)?;
    while let Some(row) = rows.next()? {
        s = row.get(0)?;
    }

    Ok(s)
}
