extern crate config;

use clap::Parser;
use futures_channel::mpsc;
use futures_util::{SinkExt, StreamExt};
use matrix_bot_tester::args::Args;

use anyhow::anyhow;

use matrix_sdk::{
    self,
    config::SyncSettings,
    event_handler::EventHandlerHandle,
    room,
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

#[derive(Debug)]
enum RoomToConnect {
    DM,
    /*
    TASK: Implement connecting to a specific room.
    TASK_ID: 809e6f8b327944b146161f698ffd50db
    CREATED: 2022-09-23 11:10
    ESTIMATED_TIME: W3
    */
    Room(String),
    WaitForMessage,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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

    println!(
        "Logging in to homeserver {} as {}.",
        &homeserver_url_str, &user
    );
    client.login_username(&user, &password).send().await?;
    println!("Logged in successfully.");
    println!("Syncing data...");
    client.sync_once(SyncSettings::new()).await?;
    println!("Sync complete.");
    let room_to_connect: RoomToConnect = match replay.get_string("room_to_connect").as_deref() {
        Ok("!dm") => RoomToConnect::DM,
        Err(_) => RoomToConnect::DM,
        Ok("!wait_for_message") => RoomToConnect::WaitForMessage,
        Ok(_room_name) => todo!("Implement the ability to specify a room to connect to."), //RoomToConnect::Room{room_name},
    };

    let dm_room = match room_to_connect {
        RoomToConnect::DM => {
            let user_id_string: String = settings.get_string("user_to_test")?;
            println!("Creating a dm room with user \"{}\".", &user_id_string);
            let user_id: OwnedUserId = UserId::parse(&user_id_string)?;
            let dm_room = client.create_dm_room(user_id).await?;
            assert!(!dm_room.is_none());
            dm_room
        }
        RoomToConnect::WaitForMessage => None,
        RoomToConnect::Room(_) => todo!("Implement specifying which room to connect to."),
    };

    let mut room_were_talking_in = dm_room;

    let (tx, mut rx) = mpsc::channel::<(room::Joined, String)>(1);

    let handle = client.add_event_handler({
        move |event: OriginalSyncRoomMessageEvent, room: Room| {
            let mut tx = tx.clone();
            async move {
                /*
                TASK: Differenciate between rooms when acting as client.
                TASK_ID: d8a25b06580bb9b6605fb8ad0d5c8c31
                CREATED: 2022-09-23 10:27
                ESTIMATED_TIME: W4
                 */

                if let Room::Joined(room) = room {
                    match event.content.msgtype {
                        MessageType::Text(TextMessageEventContent { body, .. }) => {
                            tx.send((room, body)).await.unwrap();
                        }
                        _ => (),
                    }
                } else {
                    panic!("room: {:?}, event {:?}", room, event);
                }
            }
        }
    });

    let _guard = event_handler_drop_guard(client, handle);
    let messages = replay.get_array("messages").unwrap();
    println!("{:?}", messages);
    for message in messages {
        let message_pair = message.into_table().unwrap();
        if let Some(send) = message_pair.get("send") {
            let send = send.clone();
            let send = send.into_string()?;
            println!("send: {}", send);
            let content = RoomMessageEventContent::text_plain(&send);
            let txn_id = TransactionId::new();
            room_were_talking_in
                .as_ref()
                .ok_or(anyhow!("No room specified as to where to send the message"))?
                .send(content, Some(&txn_id))
                .await?;
        };
        if let Some(expectation) = message_pair.get("expect") {
            let expectation = expectation.clone();
            let expectation: String = expectation.into_string()?;
            let response = loop {
                let (room, response) = StreamExt::next(&mut rx).await.expect("next room message.");
                let right_room = match room_were_talking_in.as_ref() {
                    Some(rwti) => room.room_id().as_str() == rwti.room_id(),
                    None => {
                        room_were_talking_in = Some(room);
                        true
                    }
                };
                if right_room {
                    break response;
                }
            };
            println!("recieved: {}", response);
            if response != expectation {
                eprintln!("Expected to hear '{}'", expectation);
                std::process::exit(1);
            }
        };
    }
    Ok(())
}
