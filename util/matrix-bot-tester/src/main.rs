extern crate config;

use matrix_bot_tester::args::{Args};
use clap::Parser;
use std::process::{exit};
use std::collections::BTreeMap;

use matrix_sdk_common::instant::Duration;
use matrix_sdk::{
    self,
    //room::Room,
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
    },
    Client, config::SyncSettings,
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

    const SYNC_WAIT_TIME: Duration = Duration::from_secs(3);

    // First we create the DM room, where we invite the user and tell the
    // invitee that the room should be a DM.
    let bot_to_test_name: String = settings.get_str("user_to_test").unwrap();
    let bot_to_test = <&UserId>::try_from("foo").unwrap();
    let invite = &[<OwnedUserId>::try_from(bot_to_test.clone()).unwrap()];

    let request = assign!(ruma::api::client::room::create_room::v3::Request::new(), {
        invite,
        is_direct: true,
        preset: Some(RoomPreset::TrustedPrivateChat),
    });

    let response = client.send(request, None).await?;

    // Now we need to mark the room as a DM for ourselves, we fetch the
    // existing `m.direct` event and append the room to the list of DMs we
    // have with this user.
    let mut content = client
        .store()
        .get_account_data_event(GlobalAccountDataEventType::Direct)
        .await?
        .map(|e| e.deserialize_as::<DirectEvent>())
        .transpose()?
        .map(|e| e.content)
        .unwrap_or_else(|| ruma::events::direct::DirectEventContent(BTreeMap::new()));

    content.entry(bot_to_test.to_owned()).or_default().push(response.room_id.to_owned());

    // TODO We should probably save the fact that we need to send this out
    // because otherwise we might end up in a state where we have a DM that
    // isn't marked as one.
    client.send_account_data(content).await?;

    // If the room is already in our store, fetch it, otherwise wait for a
    // sync to be done which should put the room into our store.
    let room = if let Some(room) = client.get_joined_room(&response.room_id) {
        room
    } else {
        //client.inner.sync_beat.listen().wait_timeout(SYNC_WAIT_TIME);
        client.get_joined_room(&response.room_id).unwrap()
    };


    return Ok(());
    //client.create_dm_room(settings.get_str("user_to_test").unwrap()).await?;

    /*
    let mut room = bot.create_and_join_room(inviting=[settings.get_str("user_to_test").unwrap()], private=true);

    bot.run(&user, &password, &homeserver_url);

    let messages = replay.get_array("messages").unwrap();
    for message in messages {
        let message_pair = message.into_table().unwrap();
        if let send = message_pair.get("send") {
            println!("send: {}", send);
            bot.send_to_user(message_pair.get("send").unwrap(), args.user_to_test);
        };
        if let expectation = message_pair.get("expect"){
            let response = bot.recieve_from_user(args.user_to_test);
            println!("recieved: {}", response);
            if response != expectation {
                eprintln!("Expected to hear '{}'", expectation);
                std::process::exit(1);
            }

        };
    }
*/

}
