use serverbrowse::protocol::*;
use tokio::net::UdpSocket;
use std::vec;

mod network;

use crate::network::*;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("could not bind socket");

    send_master_request(&socket, "master4.teeworlds.com:8300").await;

    let mut addr_list: Vec<Addr6Packed> = vec![];
    let mut server_count: Option<u16> = None;

    recieve_master_results(&socket, &mut addr_list, &mut server_count).await;
    
    println!("{:?}", server_count);

    let mut handles = vec![];
    let mut results: Vec<Result<Option<ServerInfo>, tokio::task::JoinError>> = vec![];

    let mut i = 0;
    for addr in addr_list.as_slice() {
        let handle = tokio::spawn(send_recieve_server(i, addr.unpack().to_string(), 1000));
        handles.push(handle);
        println!("{}", i);
        i += 1;
        if i % 80 == 0 || usize::try_from(i).unwrap() == addr_list.len() {
            results.append(&mut futures::future::join_all(&mut handles).await);
            handles.clear();
        }
    }

    let mut server_infos = 0;
    let mut none_server_infos = 0;

    for result in results {
        match result {
            Ok(x) => {
                match x {
                    Some(server_info) => {
                        //println!("Map: {}", server_info.map);
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
}
