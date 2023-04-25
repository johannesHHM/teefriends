use serverbrowse::protocol::*;
use std::vec;

mod network;
mod settings;
use crate::network::*;
use crate::settings::*;

async fn fetch_friend_data(online_friends: &mut Vec<String>, settings_path: String) {
    let mut addr_list: Vec<Addr6Packed> = vec![];

    send_recieve_masters("master4.teeworlds.com:8300", &mut addr_list).await;
    send_recieve_masters("master3.teeworlds.com:8300", &mut addr_list).await;
    println!("Done with masters");

    let mut handles = vec![];
    let mut results: Vec<Result<Option<ServerInfo>, tokio::task::JoinError>> = vec![];

    let mut i = 0;
    for addr in addr_list.as_slice() {
        let handle = tokio::spawn(send_recieve_server(i, addr.unpack().to_string(), 1000));
        handles.push(handle);
        i += 1;
        if i % 80 == 0 || usize::try_from(i).unwrap() == addr_list.len() {
            results.append(&mut futures::future::join_all(&mut handles).await);
            handles.clear();
        }
    }
    println!("Done with sending");

    let mut online_players: Vec<String> = vec![];

    for result in results {
        match result {
            Ok(x) => {
                match x {
                    Some(server_info) => {
                        for client in server_info.clients {
                            online_players.push(client.name.to_string());
                        }
                    },
                    None => (),
                }
            }
            Err(_) => (),
        }
    }

    let friends = read_friends(settings_path).unwrap();

    for player in online_players {
        for friend in &friends {
            if player.eq(friend) {
                online_friends.push(friend.to_string());
            }
        }
    }
}

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
    let mut online_friends: Vec<String> = vec![];
    //fetch_friend_data(&mut online_friends, "/home/johannes/.local/share/ddnet/settings_ddnet.cfg".to_string()).await;
    //store_data(&online_friends, "/home/johannes/.local/share/teefriends/friends.txt".to_string()).expect("");
    read_store_data(&mut online_friends, "/home/johannes/.local/share/teefriends/friends.txt".to_string()).expect("");

    print_active_friends(&online_friends);
    print_active_friend_count(&online_friends);

    dbg!(online_friends);
}
