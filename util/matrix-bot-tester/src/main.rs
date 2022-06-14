extern crate config;

use matrix_bot_tester::args::{Args};
use clap::Parser;
use std::process::{exit};

use matrix_sdk::{
    self,
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        SyncMessageEvent,
    },
    Client, SyncSettings,
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
    let client = Client::new(homeserver_url).unwrap();

    // client.register_event_handler(on_room_message).await;

    println!("Logging in to homeserver {} as {}", &homeserver_url_str, &user);
    match client.login(&user, &password, None, None).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error logging in {}", e);
            exit(1);
        }
    };
    println!("Logged in successfully.");
    println!("Syncing data");
    client.sync_once(SyncSettings::new()).await?;
    println!("Sync complete");
    return Ok(());

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
