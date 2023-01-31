use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use tokio::time::{sleep, Duration};

use matrix_sdk::{self, room::Room, Client};

// Taken from https://github.com/matrix-org/matrix-rust-sdk/blob/3d22b6d5a407601d9b77e99ab4d95d726aa47366/examples/autojoin/src/main.rs#L8
pub async fn autojoin_rooms_event_handler(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        tokio::spawn(async move {
            println!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = room.accept_invitation().await {
                // retry autojoin due to synapse sending invites, before the
                // invited user can join for more information see
                // https://github.com/matrix-org/synapse/issues/4345
                eprintln!(
                    "Failed to join room {} ({err:?}), retrying in {delay}s",
                    room.room_id()
                );

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    eprintln!("Can't join room {} ({err:?})", room.room_id());
                    break;
                }
            }
            println!("Successfully joined room {}", room.room_id());
        });
    }
}
