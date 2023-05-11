use std::vec;
use clap::{Command, Arg, ArgAction};

mod network;
mod settings;
use crate::network::*;
use crate::settings::*;

use std::process::exit;

fn print_active_friends(online_friends: &Vec<String>) {
    for friend in online_friends {
        println!("{}", friend);
    }
}
fn print_active_friend_count(online_friends: &Vec<String>) {
    println!("{}", online_friends.len());
}

fn main() {
    let matches = Command::new("teefriends")
        .version("0.1.0")
        .author("JohannesHHM")
        .about("Checks servers for online friends")
        .arg_required_else_help(true)
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Print friend count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("names")
                .short('n')
                .long("names")
                .help("Print friend names")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fetch")
                .short('f')
                .long("fetch")
                .help("Update friend storage")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let data_dir = get_data_dir();

    if data_dir.is_none() {
        println!("[ERROR] Could not get data directory!");
        exit(1);
    }

    let data_dir = data_dir.unwrap();

    let mut online_friends: Vec<String> = vec![];
    let store_path = String::from(data_dir.clone() + "/teefriends");
    let settings_path = String::from(data_dir.clone() + "/ddnet/settings_ddnet.cfg");

    if matches.get_flag("fetch") {
        fetch_friend_data(&mut online_friends, &settings_path).expect("");
        match store_data(&online_friends, &store_path) {
            Ok(()) => (),
            Err(_) => println!("[ERROR] Could not write to store data!"),
        }
        online_friends.clear();
    }

    if matches.get_flag("names") {
        match read_store_data(&mut online_friends, &store_path) {
            Ok(()) => print_active_friends(&online_friends),
            Err(_) => println!("[ERROR] Could not read from store data!"),
        }
        online_friends.clear();
    }

    if matches.get_flag("count") {
        match read_store_data(&mut online_friends, &store_path) {
            Ok(()) => print_active_friend_count(&online_friends),
            Err(_) => println!("[ERROR] Could not read from store data!"),
        }
        online_friends.clear();
    }
}
