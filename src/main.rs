use serverbrowse::protocol::*;
use std::vec;

mod network;
mod settings;

use crate::network::*;
use crate::settings::*;

#[tokio::main]
async fn main() {
    let mut addr_list: Vec<Addr6Packed> = vec![];

    send_recieve_masters("master4.teeworlds.com:8300", &mut addr_list).await;
    send_recieve_masters("master3.teeworlds.com:8300", &mut addr_list).await;

    //println!("Address list length: {}", addr_list.len());

    let mut handles = vec![];
    let mut results: Vec<Result<Option<ServerInfo>, tokio::task::JoinError>> = vec![];

    let mut i = 0;
    for addr in addr_list.as_slice() {
        let handle = tokio::spawn(send_recieve_server(i, addr.unpack().to_string(), 1000));
        handles.push(handle);
        //println!("{}", i);
        i += 1;
        if i % 80 == 0 || usize::try_from(i).unwrap() == addr_list.len() {
            results.append(&mut futures::future::join_all(&mut handles).await);
            handles.clear();
        }
    }

    let mut _server_infos = 0;
    let mut _none_server_infos = 0;
    let mut online_players: Vec<String> = vec![];

    for result in results {
        match result {
            Ok(x) => {
                match x {
                    Some(server_info) => {
                        for client in server_info.clients {
                            online_players.push(client.name.to_string());
                        }
                        _server_infos += 1;
                    },
                    None => {
                        _none_server_infos +=1;
                    },
                }
            }
            Err(_) => (),
        }
    }

    //println!("Infos: {}, None: {}", server_infos, none_server_infos);

    let friends = read_friends("/home/johannes/.local/share/ddnet/settings_ddnet.cfg".to_string()).unwrap();

    let mut online_friends: Vec<String> = vec![];

    for player in online_players {
        for friend in &friends {
            if player.eq(friend) {
                online_friends.push(friend.to_string());
            }
        }
    }
    for friend in &online_friends {
        println!("{}", friend);
    }
}
