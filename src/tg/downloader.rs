
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


pub async fn dl_thumb_to_disk_old(
    caller: &mut Caller,
    t: &types::MediaThumb,
) -> Result<(), GenErr> {
    // hack: use Media for dl
    let mut m = types::Media::default();
    m.dep_volume_id = t.dep_volume_id;
    m.dep_local_id = t.dep_local_id;
    m.w = t.w;
    m.h = t.h;
    m.size = t.size;
    m.media_type = types::MediaType::Image;
    let res = _dl_image(caller, m.clone()).await?;
    std::fs::create_dir_all("./_dl_thumb/").unwrap();
    let name = format!("./_dl_thumb/{}{}", m.id, m.file_extention);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&res);
    Ok(())
}

pub async fn dl_media_thumb_to_disk(caller: &mut Caller, m: types::Media) -> Result<(), GenErr> {
    // let o = *m.video_thumbs_rec;
    if let Some(t) = *m.video_thumbs_rec {
        // println!("++++ Downloading video thumb {}{}", o. );
        let res = _dl_file(caller, t.clone()).await?;
        std::fs::create_dir_all("./_dl_thumb/").unwrap();
        let name = format!("./_dl_thumb/{}.{}", t.id, t.file_extention);
        let mut f = std::fs::File::create(name).unwrap();
        f.write(&res);
    };
    Ok(())
}

pub async fn dl_media(caller: &mut Caller, m: types::Media) -> Result<Vec<u8>, GenErr> {
    use types::MediaType::*;
    match m.media_type {
        Image => {
            _dl_image(caller, m.clone()).await
            // let res = _dl_image(caller,m.clone()).await.unwrap();
            // std::fs::create_dir_all("./_dl/").unwrap();
            // let name = format!("./_dl/{}{}", m.id,m.file_extention);
            // let mut f = std::fs::File::create(name).unwrap();
            // f.write(&res);
        }
        Video | Audio | File | ImageFile => {
            _dl_file(caller, m.clone()).await
            // let res = _dl_file(caller,m.clone()).await.unwrap();
            // std::fs::create_dir_all("./_dl/").unwrap();
            // let name = format!("./_dl/{}{}", m.id,m.file_extention);
            // let mut f = std::fs::File::create(name).unwrap();
            // f.write(&res);
        }
        Unknown => Err(GenErr::Download),
    }
}

async fn _dl_image(caller: &mut Caller, m: types::Media) -> Result<Vec<u8>, GenErr> {
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::InputPhotoFileLocation(
            tl::types::InputPhotoFileLocation {
                id: m.id,
                access_hash: m.access_hash,
                file_reference: m.file_reference,
                thumb_size: m.photo_size_type,
            },
        ),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await?;

    let mut out = vec![];
    use tl::enums::upload::File;
    match res {
        File::File(tfile) => {
            // f.write(&tfile.bytes);
            out.write(&tfile.bytes);
        }
        File::CdnRedirect(red) => {
            println!("cdn redirect");
        }
    };
    Ok(out)
}

async fn _dl_file(caller: &mut Caller, m: types::Media) -> Result<Vec<u8>, GenErr> {
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    loop {
        let request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation(
                tl::types::InputDocumentFileLocation {
                    id: m.id,
                    access_hash: m.access_hash,
                    file_reference: m.file_reference.clone(),
                    thumb_size: m.photo_size_type.clone(), // todo fix me
                },
            ),
            offset: offset,
            limit: limit,
        };
        let res = send_req(caller, &request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        break;
                    }
                };
            }
            Err(err) => {
                break;
            }
        }
    }

    if out_buffer.len() == 0 {
        return Err(GenErr::Download);
    }

    Ok(out_buffer)
}

async fn get_file(caller: &mut Caller, req: tl::types::InputFileLocation) {
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::Location(req),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await.unwrap();
    // println!("get_chat_id:  {:#?}", res);
}

async fn get_file_photo(caller: &mut Caller, req: tl::types::InputPhotoFileLocation) {
    // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  {:#?}", req);
    let request = tl::functions::upload::GetFile {
        precise: false,
        cdn_supported: false,
        location: tl::enums::InputFileLocation::InputPhotoFileLocation(req.clone()),
        offset: 0,
        limit: 524288,
    };
    let res = send_req(caller, &request).await.unwrap();

    std::fs::create_dir_all("./out/").unwrap();
    let name = format!("./out/{}.jpg", req.id);
    let mut f = std::fs::File::create(name).unwrap();

    use tl::enums::upload::File;

    match res {
        File::File(tfile) => {
            f.write(&tfile.bytes);
        }
        File::CdnRedirect(red) => {}
    };
}

async fn get_file_doc(caller: &mut Caller, req: tl::types::InputDocumentFileLocation) {
    let limit = 524288;
    let mut out_buffer = Vec::with_capacity(limit as usize);
    let mut offset = 0;

    loop {
        let request = tl::functions::upload::GetFile {
            precise: false,
            cdn_supported: false,
            location: tl::enums::InputFileLocation::InputDocumentFileLocation(req.clone()),
            offset: offset,
            limit: limit,
        };
        let res = send_req(caller, &request).await;

        match res {
            Ok(res) => {
                use tl::enums::upload::File;
                match res {
                    File::File(tfile) => {
                        let len = tfile.bytes.len() as i32;
                        out_buffer.write(&tfile.bytes);
                        if len == limit {
                            offset = offset + limit;
                        } else {
                            break;
                        }
                    }
                    File::CdnRedirect(red) => {
                        break;
                    }
                };
            }
            Err(err) => {
                break;
            }
        }
        //println!("%%%%%% get_file_photo :  {:#?}", res);
    }

    if out_buffer.len() == 0 {
        return;
    }

    std::fs::create_dir_all("./out/").unwrap();
    let name = format!("./out/{}.file", req.id);
    let mut f = std::fs::File::create(name).unwrap();
    f.write(&out_buffer);
}


async fn send_req<R: RemoteCall>(
    caller: &mut Caller,
    request: &R,
) -> Result<R::Return, InvocationError> {
    caller.client.invoke(request).await
}
