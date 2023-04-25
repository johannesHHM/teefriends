use std::vec;
use clap::{Command, Arg, ArgAction};

mod network;
mod settings;
use crate::network::*;
use crate::settings::*;

fn print_active_friends(online_friends: &Vec<String>) {
    for friend in online_friends {
        println!("{}", friend);
    }
}
fn print_active_friend_count(online_friends: &Vec<String>) {
    println!("{}", online_friends.len());
}

#[tokio::main]
async fn main() {
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

    let mut online_friends: Vec<String> = vec![];

    if matches.get_flag("fetch") {
        fetch_friend_data(&mut online_friends, "/home/johannes/.local/share/ddnet/settings_ddnet.cfg".to_string()).await;
        store_data(&online_friends, "/home/johannes/.local/share/teefriends/friends.txt".to_string()).expect("");
        online_friends.clear();
    }

    if matches.get_flag("names") {
        read_store_data(&mut online_friends, "/home/johannes/.local/share/teefriends/friends.txt".to_string()).expect("");
        print_active_friends(&online_friends);
        online_friends.clear();
    }

    if matches.get_flag("count") {
        read_store_data(&mut online_friends, "/home/johannes/.local/share/teefriends/friends.txt".to_string()).expect("");
        print_active_friend_count(&online_friends);
        online_friends.clear();
    }
}
