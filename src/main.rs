use serverbrowse::protocol::*;
use std::vec;
use std::{thread, time};

mod network;
mod settings;

use crate::network::*;
use crate::settings::*;

#[tokio::main]
async fn main() {

    /* let socket = UdpSocket::bind("0.0.0.0:0").await.expect("could not bind socket");

    send_master_request(&socket, "master4.teeworlds.com:8300").await;
    let mut server_count = 0;

    let mut addr_list: Vec<Addr6Packed> = vec![];
    let mut server_count4: Option<u16> = None;

    recieve_master_results(&socket, &mut addr_list, &mut server_count4).await;
    println!("Sending 3");
    send_master_request(&socket, "master3.teeworlds.com:8300").await;

    //let mut addr_list: Vec<Addr6Packed> = vec![];
    let mut server_count3: Option<u16> = None;

    recieve_master_results(&socket, &mut addr_list, &mut server_count3).await;
    println!("Recieving 3");

    
    server_count += server_count4.unwrap() + server_count3.unwrap();
    println!("{:?}", server_count);
    println!("{}", addr_list.len()); */

    //let mut handles_masters = vec![];


    let mut addr_list: Vec<Addr6Packed> = vec![];

    send_recieve_masters("master4.teeworlds.com:8300", &mut addr_list).await;

    println!("Address length: {}", addr_list.len());
    
    send_recieve_masters("master3.teeworlds.com:8300", &mut addr_list).await;

    println!("Address length: {}", addr_list.len());


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

    let mut server_infos = 0;
    let mut none_server_infos = 0;
    let mut online_players: Vec<String> = vec![];

    for result in results {
        match result {
            Ok(x) => {
                match x {
                    Some(server_info) => {
                        for client in server_info.clients {
                            online_players.push(client.name.to_string());
                        }
                        server_infos += 1;
                    },
                    None => {
                        none_server_infos +=1;
                    },
                }
            }
            Err(_) => (),
        }
    }

    println!("Infos: {}, None: {}", server_infos, none_server_infos);

    let friends = read_friends("/home/johannes/.local/share/ddnet/settings_ddnet.cfg".to_string()).unwrap();

    /* println!("PRINTING PLAYERS");
    for player in &online_players {
        println!("{}", player);
    }

    println!("PRINTING FRIENDS");
    for friend in &friends {
        println!("{}", friend);
    } */

    let mut online_friends: Vec<String> = vec![];

    for player in online_players {
        for friend in &friends {
            if player.eq(friend) {
                online_friends.push(friend.to_string());
            }
        }
    }

    println!("PRINTING ONLINE FRIENDS");
    for friend in &online_friends {
        println!("{}", friend);
    }

}
