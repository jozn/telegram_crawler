// use sqlite;
use crate::types;

use rusqlite::{params, Connection, Result};

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
    if username.is_empty() || !username.is_ascii() {
        return;
    }
    let q = "insert into queue_username (username) values (?1)";
    con.execute(q, params![username]).unwrap();
}

pub fn delete_queue_username(username: &str) {
    let con = get_conn();
    let q = "delete form queue_username where username = ?1";
    con.execute(q, params![username]).unwrap();
}

pub fn save_cached_username(cud: &types::CachedUsernameData) {
    let con = get_conn();
    let bin = serde_json::to_string(cud).unwrap();
    let q = "insert into cached_username (username, channel_id ,data) values (?1,?2,?3)";
    con.execute(q, params![&cud.username, cud.channel_id, bin])
        .unwrap();
}

pub fn load_all_cached_usernames() {}

fn get_conn() -> Connection {
    let con = Connection::open("./crawling.sqlite").unwrap();
    con.execute("PRAGMA synchronous = OFF", params![]);
    con
}

pub fn main2() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        params![],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
