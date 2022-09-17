extern crate config;

use clap::Parser;
use futures_channel::mpsc;
use futures_util::{SinkExt, StreamExt};
use matrix_bot_tester::args::Args;
use std::sync::Arc;

use matrix_sdk::{
    self,
    config::SyncSettings,
    event_handler::EventHandlerHandle,
    room::Room,
    ruma::events::room::message::{
        MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent, TextMessageEventContent,
    },
    ruma::{OwnedUserId, TransactionId, UserId},
    Client,
};
use url::Url;

#[derive(Debug)]
pub struct EventHandlerDropGuard {
    handle: EventHandlerHandle,
    client: Client,
}

#[allow(dead_code)]
pub(crate) fn event_handler_drop_guard(
    client: Client,
    handle: EventHandlerHandle,
) -> EventHandlerDropGuard {
    EventHandlerDropGuard {
        client: client.clone(),
        handle,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (tx, mut rx) = mpsc::channel::<String>(1);

    let settings = config::Config::builder()
        .add_source(config::File::new(
            &args.bot_config,
            config::FileFormat::Yaml,
        ))
        .build()?;

    let replay = config::Config::builder()
        .add_source(config::File::new(&args.replay, config::FileFormat::Yaml))
        .build()?;

    // Login
    let user = settings.get_string("user")?;
    let password = settings.get_string("password")?;
    let homeserver_url_str = settings.get_string("homeserver_url")?;
    let homeserver_url =
        Url::parse(&homeserver_url_str).expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url).await?;

    // client.register_event_handler(on_room_message).await;

    println!(
        "Logging in to homeserver {} as {}.",
        &homeserver_url_str, &user
    );
    client.login_username(&user, &password).send().await?;
    println!("Logged in successfully.");
    println!("Syncing data...");
    client.sync_once(SyncSettings::new()).await?;
    println!("Sync complete.");
    let user_id_string: String = settings.get_string("user_to_test")?;
    let user_id: OwnedUserId = UserId::parse(&user_id_string)?;
    let dm_room = Arc::new(client.create_dm_room(&user_id).await?);
    let dm_room_closure = dm_room.clone();

    let handle = client.add_event_handler({
        move |event: OriginalSyncRoomMessageEvent, room: Room| {
            let mut tx = tx.clone();
            let room_id = dm_room_closure.room_id();

            async move {
                if let Room::Joined(room) = room {
                    if room.room_id() == room_id {
                        match event.content.msgtype {
                            MessageType::Text(TextMessageEventContent { body, .. }) => {
                                tx.send(body).await.unwrap();
                            }
                            _ => return,
                        }
                    }
                }
            }
        }
    });

    let _guard = event_handler_drop_guard(client, handle);
    let messages = replay.get_array("messages").unwrap();
    for message in messages {
        let message_pair = message.into_table().unwrap();
        if let Some(send) = message_pair.get("send") {
            let send = send.clone();
            let send = send.into_string()?;
            println!("send: {}", send);
            let content = RoomMessageEventContent::text_plain(&send);
            let txn_id = TransactionId::new();
            dm_room.send(content, Some(&txn_id)).await?;
        };
        if let Some(expectation) = message_pair.get("expect") {
            let expectation = expectation.clone();
            let expectation: String = expectation.into_string()?;
            let response = StreamExt::next(&mut rx).await.expect("next room message.");
            println!("recieved: {}", response);
            if response != expectation {
                eprintln!("Expected to hear '{}'", expectation);
                std::process::exit(1);
            }
        };
    }
    Ok(())
}
