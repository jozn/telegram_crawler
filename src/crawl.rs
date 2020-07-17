use grammers_client::Client;
use grammers_tl_types::Serializable;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::ops::Index;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::delay_for;

use crate::{db, errors::GenErr, tg, types, utils};

pub async fn crawl_next_username() -> Result<(), GenErr> {
    let mut caller = get_caller().await;
    let username = db::get_next_queue_username()?;
    // let username = "flip_app".to_string();

    let rpc_res = tg::get_channel_by_username(&mut caller, &username).await;

    println!("res >> {:#?}", rpc_res);

    // Save free username [free = USERNAME_NOT_OCCUPIED or registered for personal users ]
    let save_free = |taken: bool| {
        let cud = types::CachedUsernameData {
            username: username.clone(),
            channel_id: 0,
            channel_info: None,
            taken: taken,
            last_checked: utils::time_now_sec(),
        };
        db::save_cached_username(&cud);
        db::delete_queue_username(&username);
    };

    let cud = match rpc_res {
        Ok(chan_res) => {
            let chanel_info =
                tg::get_channel_info(&mut caller, chan_res.id, chan_res.access_hash).await?;
            let channel_info_clone = chanel_info.clone();

            let cud = types::CachedUsernameData {
                username: chanel_info.username,
                channel_id: chanel_info.id,
                channel_info: Some(channel_info_clone),
                taken: true,
                last_checked: utils::time_now_sec(),
            };
            db::save_cached_username(&cud);
            db::delete_queue_username(&username);
        }

        Err(e) => match e {
            GenErr::TGRPC(rpc) => {
                if rpc.code == 400 && &rpc.name == "USERNAME_NOT_OCCUPIED" {
                    save_free(false);
                }
            }
            GenErr::TGConverter => {
                // This means username is used in other places: personal accounts,...
                save_free(true);
            }
            _ => {}
        },
    };

    // delay_for(Duration::from_millis(5000)).await;
    Ok(())
}

pub async fn crawl_config() {
    let mut caller = get_caller().await;
    println!("Getting config ... ");
    delay_for(Duration::from_millis(4000)).await;
    let res = tg::get_configs(&mut caller).await;

    println!("res >> {:#?}", res);
}

pub async fn get_caller() -> tg::Caller {
    let con = crate::con_mgr::get_new_session().await.unwrap();
    let caller = tg::Caller { client: con };
    caller
}

pub async fn crawl_next_channel_messages() -> Result<(), GenErr> {
    let mut caller = get_caller().await;
    let cd = db::get_random_cached_channel()?;
    if let Some(ci) = cd.channel_info {
        println!("senfing ");

        let req = tg::ReqGetMessages {
            channel_id: ci.id,
            access_hash: ci.access_hash,
            offset_id: 0,
            offset_date: 0,
            add_offset: 0,
            limit: 100,
            max_id: 0,
            min_id: 0,
            hash: 0,
        };

        let rpc_res = tg::get_messages(&mut caller, req).await;

        println!("channel: {:#?} \n\n  res >> {:#?}", ci, rpc_res);

        //// Dl medias
        let r = rpc_res.unwrap();
        if true {
            for m in r.msgs {
                // dl media
                /*if let Some(f) = m.media {
                    println!("++++ Downloading file {}{}", f.id, f.file_extention);
                    let t = tg::dl_media_to_disck(&mut caller, &f).await;
                    println!("--- result {:?}", t);
                }*/

                // dl humb
                if let Some(f) = m.media {
                    println!("++++ Downloading thumbs file {}{}", f.id, f.file_extention);
                    let t = tg::dl_media_thumb_to_disk(&mut caller, f).await;
                    println!("--- result {:?}", t);
                }

                /*// dl video thumb -- old
                if let Some(f) = m.media {
                    if let Some(t) = f.video_thumbs {
                        println!("++++ Downloading video thumb {}{}", f.id, f.file_extention);
                        let t = tg::dl_thumb_to_disk_old(&mut caller, &t).await;
                        println!("--- result {:?}", t);
                    }
                }*/

                /*// dl webpage photo
                if let Some(v) = m.webpage {
                    if let Some(f) = v.photo {
                        println!("++++ Downloading webpage file file {}{}", f.id, f.file_extention);
                        let t = tg::dl_media_to_disck(&mut caller, &f).await;
                        println!("--- result {:?}", t);
                    }
                }*/
            }
        }

        delay_for(Duration::from_millis(5000)).await;
    }

    Ok(())
}

//////////////////////////// Archive /////////////////////////////

pub async fn crawl_next_channel_dep() {
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
            // println!("> {}", super::get_next_channel_username());
        }
    }
}
