mod work_table;
mod ageing_cellar;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Matrix geographic user index")
        .version("0.0")
        .author("Timothy Hobbs")
        .about("A matrix bot for indexing and searching matrix users by geographic location and add rating system.")
        .get_matches();
}

