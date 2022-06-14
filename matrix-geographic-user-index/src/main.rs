mod work_table;
mod ageing_cellar;

extern crate clap;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use matrix_geographic_user_index::models::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use clap::{App, Arg, SubCommand};


// This is not a hard dependency.
// Just used for loading the username, password and homeserverurl from a file.
extern crate config;

extern crate matrix_bot_api;
use matrix_bot_api::handlers::{extract_command, HandleResult, Message, MessageHandler};
use matrix_bot_api::{ActiveBot, MatrixBot, MessageType};

// Our handler wants a mutable state (here represented by a little counter-variable)
// This counter can be increased or decreased by users giving the bot a command.
pub struct CounterHandler {
    counter: i32,
}

impl CounterHandler {
    fn new() -> CounterHandler {
        CounterHandler { counter: 0 }
    }
}

// Implement the trait MessageHandler, to be able to give it to our MatrixBot.
// This trait only has one function: handle_message() and will be called on each
// new (text-)message in the room the bot is in.
impl MessageHandler for CounterHandler {
    fn handle_message(&mut self, bot: &ActiveBot, message: &Message) -> HandleResult {
        // extract_command() will split the message by whitespace and remove the prefix (here "!")
        // from the first entry. If the message does not start with the given prefix, None is returned.
        let command = match extract_command(&message.body, "!") {
            Some(x) => x,
            None => return HandleResult::ContinueHandling,
        };

        // Now we have the current command (some text prefixed with our prefix !)
        // Your handler could have a HashMap with the command as the key
        // and a specific function for it (like StatelessHandler does it),
        // or you can use a simple match-statement, to act on the given command:
        match command {
            "incr" => self.counter += 1,
            "decr" => self.counter -= 1,
            "show" => bot.send_message(
                &format!("Counter = {}", self.counter),
                &message.room,
                MessageType::RoomNotice,
            ),
            "shutdown" => bot.shutdown(),
            _ => return HandleResult::ContinueHandling, /* Not a known command */
        }
        HandleResult::StopHandling
    }
}

fn main() {
    let matches = App::new("Matrix geographic user index")
        .version("0.0")
        .author("Timothy Hobbs")
        .about("A matrix bot for indexing and searching matrix users by geographic location and add rating system.")
        .get_matches();

    dotenv().ok();

    // let database_url = env::var("DATABASE_URL")
    //     .expect("DATABASE_URL must be set");
    // let connection = PgConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));

    // use matrix_geographic_user_index::schema::posts::dsl::*;

    // use matrix_geographic_user_index::schema::posts;
    // let new_post = NewPost {
    //     title: "Baf",
    //     body: "Loreum ipsum",
    // };

    // diesel::insert_into(posts::table)
    //     .values(&new_post)
    //     .execute(&connection)
    //     .expect("Error saving new post");

    // println!("Inserted {}", new_post.title);;

    // let results = posts
    //     .load::<Post>(&connection)
    //     .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }

    // ------- Getting the login-credentials from file -------
    // You can get them however you like: hard-code them here, env-variable,
    // tcp-connection, read from file, etc. Here, we use the config-crate to
    // load from botconfig.toml.
    // Change this file to your needs, if you want to use this example binary.
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("test-data/bot-config.toml"))
        .unwrap();

    let user = settings.get_str("user").unwrap();
    let password = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();
    // -------------------------------------------------------

    // Here we want a handler with state (simple counter-variable).
    // So we had to implement our own MessageHandler.
    let handler = CounterHandler::new();

    // Give the handler to your new bot
    let bot = MatrixBot::new(handler);

    // Blocking call (until shutdown). Handles all incoming messages and calls the associated functions.
    // The bot will automatically join room it is invited to.
    bot.run(&user, &password, &homeserver_url);
}

