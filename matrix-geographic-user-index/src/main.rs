mod work_table;
mod ageing_cellar;
/*
extern crate notify;

use notify::DebouncedEvent::Create;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use std::env;
use zmq;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn watch(path: String) -> notify::Result<()> {
    println!("Watching {}", path);
    // Create a channel to receive the events.
    let (tx_socket_dir_changes, rx_socket_dir_changes) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx_socket_dir_changes, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive)?;

    let mut sockets: Vec<zmq::Socket> = vec![];
    let ctx = zmq::Context::new();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx_socket_dir_changes.recv() {
            Ok(Create(path)) => {
                let mut socket = ctx.socket(zmq::REQ).unwrap();
                let socket_url: String = format!("ipc://{}/PAIR.zmq", path.to_str().unwrap());
                println!("Connecting to {}", socket_url);
                socket.connect(&socket_url).unwrap();
            }
            Err(e) => println!("watch error: {:?}", e),
            Ok(event) => {
                println!("Unexpected filesystem event. {:?}", event);
            }
        }
    }
}

fn main() {
    let matches = App::new("Gradesta manager")
        .version("0.0")
        .author("Timothy Hobbs <goodnight@gradesta.com>")
        .about("Manage and route messages between gradesta protocol services. Provides the meta namespace. Evaluates walk trees.")
        .get_matches();

    let mut socket_dir = String::from("");
    match env::var("HOME") {
        Ok(val) => {
            let dir: String = format!("{}/.cache/gradesta/services/", val);
            socket_dir.push_str(&dir);
        }
        Err(e) => {
            eprintln!("No $HOME directory.")
        }
    };
    watch(socket_dir);
}
*/
