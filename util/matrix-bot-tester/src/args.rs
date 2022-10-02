use clap::Parser;

// A matrix bot for testing other matrix bots. Takes a yaml file and a nick and sends the messages from the yaml file to the nick and checks whether the nick responds as expected. Exits with status code 1 if unexpected messages are recieved.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // Bot config: toml file that lists test bot's `user`, `password` and `homeserver_url`
    #[clap(short, long)]
    pub bot_config: String,

    // Toml file with messages and expected responses that should be replayed to the tested user
    #[clap(short, long)]
    pub replay: String,
}
