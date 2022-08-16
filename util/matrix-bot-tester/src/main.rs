extern crate config;

use matrix_bot_tester::args::{Args};
use clap::Parser;
use std::process::{exit};
use std::collections::BTreeMap;
use tokio::sync::mpsc;

use matrix_sdk_common::instant::Duration;
use matrix_sdk::{
    self,
    room::Room,
    ruma,
    ruma::{
        OwnedUserId,
        UserId,
        api::client::room::create_room::v3::RoomPreset,
        assign,
    },
    ruma::events::{
        //room::message::{MessageType, TextMessageEventContent},
        direct::DirectEvent,
        GlobalAccountDataEventType,
        room::message::{OriginalSyncRoomMessageEvent, RoomMessageEventContent, TextMessageEventContent},
    },
    Client, config::SyncSettings,
    uint, MilliSecondsSinceUnixEpoch, TransactionId,
};
use url::Url;


#[tokio::main]
async fn main() -> Result<(), matrix_sdk::Error>  {
    let args = Args::parse();

    let mut settings = config::Config::default();
    match settings.merge(config::File::with_name(&args.bot_config)) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error loading settings file: {}", e);
                exit(1);
            }
    };

    let mut replay = config::Config::default();
    match replay.merge(config::File::with_name(&args.replay)){
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error loading replay file: {}", e);
            exit(1);
        }
    };

    // Login
    let user = settings.get_str("user").unwrap();
    let password = settings.get_str("password").unwrap();
    let homeserver_url_str = settings.get_str("homeserver_url").unwrap();
    let homeserver_url = Url::parse(&homeserver_url_str).expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url).await?;

    // client.register_event_handler(on_room_message).await;

    println!("Logging in to homeserver {} as {}.", &homeserver_url_str, &user);
    match client.login(&user, &password, None, None).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error logging in {}", e);
            exit(1);
        }
    };
    println!("Logged in successfully.");
    println!("Syncing data...");
    client.sync_once(SyncSettings::new()).await?;
    println!("Sync complete.");
    let room = client.create_dm_room(<&UserId>::try_from(settings.get_str("user_to_test").unwrap()).to_owned()).await?;

    let room_id = room.room_id();
    let (tx, mut rx) = mpsc::channel(32);
    client.add_event_handler(async move |event: OriginalSyncRoomMessageEvent, room: Room| {

        if let Room::Joined(room) = room {
            if room.room_id() == room_id;
            match event.content.msgtype {
                MessageType::Text(TextMessageEventContent { body, .. }) => {
                    tx.send(body);
                },
                _ => return,
            };
        }
    });

    let messages = replay.get_array("messages").unwrap();
    for message in messages {
        let message_pair = message.into_table().unwrap();
        if let send = message_pair.get("send") {
            println!("send: {}", send);
            let content = RoomMessageEventContent::text_plain(&send);
            let txn_id = TransactionId::new();
            room.send(content, Some(&txn_id)).await?;
        };
        if let expectation = message_pair.get("expect"){
            let Some(response) = rx.recv().await;
            println!("recieved: {}", response);
            if response != expectation {
                eprintln!("Expected to hear '{}'", expectation);
                std::process::exit(1);
            }
        };
    }
}
