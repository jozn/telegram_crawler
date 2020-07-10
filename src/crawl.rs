use grammers_tl_types::Serializable;
use lazy_static::lazy_static;
use rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::ops::Index;
use std::time::Duration;
use tokio::time::delay_for;

use crate::{db, errors::GenErr, tg, types, utils};

use grammers_client::Client;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub async fn crawl_next_username() {
    let mut caller = get_caller().await;
    let username = db::get_next_queue_username().unwrap();
    let username = "flip_net".to_string();

    let res = tg::get_channel_by_username(&mut caller, &username).await;

    println!("res >> {:#?}", res);

    // Save free username [free = USERNAME_NOT_OCCUPIED or registered for personal users ]
    let save_free = || {
        let cud = types::CachedUsernameData {
            username: username.clone(),
            channel_id: 0,
            tg_result: None,
            taken: true,
            last_checked: 1654,
        };
        db::save_cached_username(&cud);
    };

    let cud = match res {
        Ok(r) => {
            let r2 = r.clone();
            let cud = types::CachedUsernameData {
                username: r.username,
                channel_id: r.id,
                tg_result: Some(r2),
                taken: true,
                last_checked: 1654,
            };
            db::save_cached_username(&cud);
        }

        Err(e) => match e {
            GenErr::TGRPC(rpc) => {
                if rpc.code == 400 && &rpc.name == "USERNAME_NOT_OCCUPIED" {
                    save_free();
                }
            }
            GenErr::TGConverter => {
                // This means username is used by other places: personal accounts
                save_free();
            }
            _ => {}
        },
    };

    delay_for(Duration::from_millis(20000)).await;
}

pub async fn crawl_config() {
    let mut caller = get_caller().await;
    println!("Getting config ... ");
    delay_for(Duration::from_millis(4000)).await;
    let res = tg::get_configs(&mut caller).await;

    println!("res >> {:#?}", res);
}

pub async fn crawl_next_channel() {
    let mut caller = get_caller().await;
    for i in 0..1 {
        // let username = get_next_channel_username();
        delay_for(Duration::from_millis(20)).await;
        let res = tg::get_channel_info(&mut caller, 1072723547, -1615658883512673699).await;

        println!("res >> {:#?}", res);
    }

    GLOBAL_DATA.lock().unwrap().insert(3, "sdf".to_string());

    println!("res >> {:#?}", GLOBAL_DATA);
}

pub async fn get_caller() -> tg::Caller {
    let con = crate::con_mgr::get_new_session().await.unwrap();
    let caller = tg::Caller { client: con };
    caller
}

//////////////////////////// Archive /////////////////////////////

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        println!("dir {:?}", std::env::current_dir().unwrap());
        for i in 0..100 {
            println!("> {}", super::get_next_channel_username());
        }
    }
}
