use crate::ageing_cellar::autojoin_rooms_event_handler::autojoin_rooms_event_handler;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::SendError;

use std::process::{Command};
use std::sync::Arc;

use anyhow;

use url::Url;
use matrix_sdk::{
    self,
    config::SyncSettings,
    event_handler::EventHandlerHandle,
    room,
    room::Room,
    ruma::events::room::message::{
        MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent, TextMessageEventContent,
    },
    ruma::{OwnedUserId, TransactionId, UserId, RoomId},
    Client,
};

#[derive(Debug)]
pub struct ConnectionSettings {
    pub user: String,
    pub password: String,
    pub homeserver_url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixTextMessage {
    pub user_id: Box<UserId>,
    pub room_id: Box<RoomId>,
    pub message: String,
}

pub async fn run_matrix_addapter(connectionSettings: ConnectionSettings, tx: Sender<MatrixTextMessage>, mut rx: Receiver<MatrixTextMessage>) -> anyhow::Result<()> {
    let homeserver_url =
        Url::parse(&connectionSettings.homeserver_url).expect("Couldn't parse the homeserver URL");
    let client = Arc::new(Client::new(homeserver_url).await?);
    client
        .login_username(
            &connectionSettings.user,
            &connectionSettings.password,
        )
        .send()
        .await?;

    let me: OwnedUserId = UserId::parse(&connectionSettings.user)?;
    client.sync_once(SyncSettings::new()).await?;

    client.add_event_handler(autojoin_rooms_event_handler);

    client.add_event_handler({
        move |event: OriginalSyncRoomMessageEvent, room: Room| {

            let mut tx = tx.clone();
            let me = me.clone();
            async move {
                if let Room::Joined(room) = room {
                    match event.content.msgtype {
                        MessageType::Text(TextMessageEventContent { body, .. }) => {
                            if event.sender != me {
                                let message = MatrixTextMessage {
                                    user_id: event.sender.into(),
                                    room_id: room.room_id().into(),
                                    message: body,
                                };
                                tx.send(message).await.unwrap();
                            }
                        }
                        _ => (),
                    }
                } else {
                    panic!("room: {:?}, event {:?}", room, event);
                }
            }
        }
    });

    let client_sync = Arc::clone(&client);
    tokio::spawn(async move {
        client_sync.sync(SyncSettings::default()).await;
    });

    let client_handler = Arc::clone(&client);
    while let Some(message) = rx.recv().await {
        let room = client_handler
            .get_joined_room(&message.room_id)
            .expect("Failed to get room");

        let content = RoomMessageEventContent::text_plain(&message.message);
        let txn_id = TransactionId::new();
        room.send(content, Some(&txn_id)).await?;
    }
    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_matrix_addapter() {
        let (tx, mut rx) = mpsc::channel(100);
        let (tx2, mut rx2) = mpsc::channel(100);

        let connectionSettings = ConnectionSettings {
            user: "@bot:synapse-test.localhost".to_string(),
            password: "test".to_string(),
            homeserver_url: "http://localhost:8008".to_string(),
        };

        let handle = tokio::spawn(async move {
            run_matrix_addapter(connectionSettings, tx, rx2).await.unwrap();
        });

        // Run the matrix-bot-tester to act as the client for our bot
        let handle2 = tokio::spawn(async move {
            let mut child = Command::new("../util/matrix-bot-tester/target/debug/matrix-bot-tester")
                .arg("--bot-config")
                .arg("../util/matrix-bot-tester/test-data/client-config.yaml")
                .arg("--replay")
                .arg("../util/matrix-bot-tester/test-data/client-replay-matrix-addapter-test.yaml")
                .spawn()
                .expect("failed to execute process");

            let ecode = child.wait().expect("failed to wait on child");
            assert!(ecode.success());
        });

        let received_message = rx.recv().await.unwrap();
        assert_eq!(received_message.message, "hello");

        let message = MatrixTextMessage {
            user_id: received_message.user_id,
            room_id: received_message.room_id,
            message: "message".to_string(),
        };

        tx2.send(message).await.unwrap();

        handle.abort();
        handle2.abort();
    }
}
