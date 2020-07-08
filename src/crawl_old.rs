use std::sync::{Arc, Mutex};
use std::cell::Cell;

use crossbeam::channel::bounded;


use crate::{types, tg_old, client_pool, dbi, consumer, pipe};
pub async fn run() {
    let mut app = types::App {
        login: vec![],
        channels: Default::default(),
        sessions: vec![],
        dcs: vec![],
        clients: Mutex::new(Cell::new(client_pool::ClientPool { client: None })),
    };

    let app2 = Arc::new(app);
    // tg_old::get_contacts(&app2).await;

    let (sReqSyncChannel,rReqSyncChannel) = bounded::<types::ReqSyncChannel>(10);
    let (sResSyncChannel,rResSyncChannel) = bounded::<types::ResSyncChannel>(10);
    let (sReqSyncMessages,rReqSyncMessages) = bounded::<types::ReqSyncMessages>(10);
    let (sResSyncMessages,rResSyncMessages) = bounded::<types::ResSyncMessages>(10);
    let (sReqResolveUsername,rReqResolveUsername) = bounded::<types::ReqResolveUsername>(10);
    let (sResResolveUsername,rResResolveUsername) = bounded::<types::ResResolveUsername>(10);

    use std::thread::{spawn,Thread};

    sReqSyncChannel.send(types::ReqSyncChannel{
        channel_id: 0
    });

    sReqSyncChannel.send(types::ReqSyncChannel{
        channel_id: 0
    });

    spawn(move || {

    });

    // consumer::start_new_consumer().join();

    // tg::get_messages(&app2).await;
    //
    // threading::run();


    // tg2::get_contacts( app2.clone()).await;
    // tg::get_contacts(&app2).await;
    // tg::get_messages(&app2).await;
    // let mut app = Arc::new(Mutex::new(app));
    // let app1 = app.get_mut().unwrap();

    // app.get_dialogs().await;
    // tg::get_messages().await;
    // app.get_channel_info().await;
    // app.get_channel_by_username().await;
    // app.get_chat_id().await;
}
