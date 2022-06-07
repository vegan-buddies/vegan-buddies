extern crate config;

use matrix_bot_tester::args::{Args};
use clap::Parser;

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
    settings
        .merge(config::File::with_name(&args.bot_config))
        .unwrap();

    let mut replay = config::Config::default();
    settings
        .merge(config::File::with_name(&args.replay))
        .unwrap();

    // Login
    let user = settings.get_str("user").unwrap();
    let password = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();
    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url).unwrap();

    client.register_event_handler(on_room_message).await;

    client.login(&user, &password, None, None).await;
    client.sync(SyncSettings::new()).await;

    bot.run(&user, &password, &homeserver_url);

    let messages = replay.get_array("messages").unwrap();
    for message in messages {
        let message_pair = message.into_table().unwrap();
        let send = message_pair.get("send").unwrap();
        println!("send: {}", send);
        bot.send_to_user(message_pair.get("send").unwrap(), args.user_to_test);
        let response = bot.recieve_from_user(args.user_to_test);
        println!("recieved: {}", response);
        let expectation = message_pair.get("expect").unwrap();
        if response != expectation {
            eprintln!("Expected to hear '{}'", expectation);
            std::process::exit(1);
        }
    }

}
