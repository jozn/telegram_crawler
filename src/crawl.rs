use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use rand;
use grammers_tl_types::Serializable;
use rand::Rng;
use std::ops::Index;
use std::time::Duration;
use tokio::time::delay_for;

use crate::tg;

use std::{sync::Mutex};
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

// pub mod scheduler {
pub fn get_next_channel_username() -> String{
    let f = fs::read("./lib/play_gram1/src/tkanal.txt").unwrap();
    // let s = f.to_bytes().to_str().unwrap();
    let s = String::from_utf8(f).unwrap();
    let arr : Vec<&str>= s.split("\n").collect();
    let rnd =  rand::thread_rng().gen_range(0,arr.len());

    let kanal = arr.index(rnd).to_string();

    kanal
}

pub async fn crawl_next_user_name(){
    let mut caller = get_caller().await;
    for i in 0..1 {
        let username = get_next_channel_username();
        delay_for(Duration::from_millis(20000)).await;
        let res = tg::get_channel_by_username(&mut caller, username).await;

        println!("res >> {:#?}", res);
    }

}

pub async fn crawl_next_channel(){
    let mut caller = get_caller().await;
    for i in 0..1 {
        let username = get_next_channel_username();
        delay_for(Duration::from_millis(2000)).await;
        let res = tg::get_channel_info(&mut caller).await;

        println!("res >> {:#?}", res);
    }

    GLOBAL_DATA.lock().unwrap().insert(3,"sdf".to_string());

    println!("res >> {:#?}", GLOBAL_DATA);

}


pub async fn get_caller() -> tg::Caller {
    let con = crate::con_mgr::get_new_session().await.unwrap();
    let caller = tg::Caller {
        client: con,
    };
    caller
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_1(){
        println!("dir {:?}", std::env::current_dir().unwrap());
        for i in 0..100 {
            println!("> {}", super::get_next_channel_username());
        }
    }


}