use async_std::task;
use grammers_client::{AuthorizationError, Client, Config};
use grammers_mtsender::InvocationError;
use grammers_session as session;
use grammers_tl_types as tl;
use grammers_tl_types::enums::messages::Messages;
use grammers_tl_types::enums::{Message, MessageEntity};
use grammers_tl_types::RemoteCall;
use std::io::Write;

use crate::types;

pub async fn get_contacts(g: types::G) {
    // get contacts
    println!("lklk");
    let request = tl::functions::contacts::GetContacts { hash: 23 };

    // let mt : tl::enums::contacts::Contacts = g.clients.lock().unwrap().get_session().await.unwrap().get_mut().unwrap().invoke(&request).await.unwrap();
    // let mt : tl::enums::contacts::Contacts = g.clients.lock().unwrap();
    let mut m = g.clients.lock().unwrap();

    let mut s = m
        .get_mut()
        .get_session()
        .await
        .unwrap()
        .lock()
        .unwrap()
        .invoke(&request)
        .await
        .unwrap();
    get_con(&g, &request).await;
    // let mt : tl::enums::contacts::Contacts = ().client.invoke(&request).await.unwrap();
    // println!("contacts {:#?}", mt);
}

async fn get_con<R: RemoteCall>(g: &types::G, request: &R) -> Result<R::Return, InvocationError> {
    let mut m = g.clients.lock().unwrap();

    let mut s = m
        .get_mut()
        .get_session()
        .await
        .unwrap()
        .lock()
        .unwrap()
        .invoke(request)
        .await;
    s
}

// send_req(g,&request).await.unwrap();
