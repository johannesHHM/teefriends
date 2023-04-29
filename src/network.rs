
use serverbrowse::protocol::*;
use serverbrowse::protocol::Response::*;
use std::time::Duration;
use std::io::Error;
use tokio::time::timeout;
use tokio::net::UdpSocket as UdpSocketTokio;

use std::collections::HashMap;

use std::net::IpAddr::V4;
use std::net::IpAddr::V6;

use std::thread;

use std::net::{UdpSocket, SocketAddr};

use crate::settings::*;

pub async fn send_master_request(sock: &UdpSocketTokio, addr: &str) {
    let header_count_6: [u8; 14] = request_count();
    let header_list_6 = request_list_6();
    //sock.connect(addr).await.ok();
    sock.send_to(&header_count_6, addr).await.ok();
    sock.send_to(&header_list_6, addr).await.ok();
}

pub fn send_master_request2(sock: &UdpSocket, addr: &str) {
    let header_count_6: [u8; 14] = request_count();
    let header_list_6 = request_list_6();
    //sock.connect(addr).await.ok();
    sock.send_to(&header_count_6, addr).ok();
    sock.send_to(&header_list_6, addr).ok();
}

pub async fn recieve_master_results(sock: &UdpSocketTokio, addr_list: &mut Vec<Addr6Packed>, server_count: &mut Option<u16>) {
    'repeat: loop {
        let mut buf: [u8; 1400] = [0; 1400];
        let (buf_size2, _addr) = sock.recv_from(&mut buf).await.unwrap();
        let buf_size = Some(buf_size2);
        match buf_size {
            Some(ref size) => {
                let buf: &[u8] = &buf[0..*size];
                let response = parse_response(&buf).unwrap();
                match response {
                    Count(x) => *server_count = Some(x.0),
                    List6(x) => {
                        for addr in x.0.iter() {
                            addr_list.push(*addr);
                        }
                    },
                    _ => (),
                }
            },
            None => (),
        }
        //TODO remove
        //dbg!(&addr_list.len());
        //dbg!(&server_count);
        match server_count {
            Some(ref count) => {
                if addr_list.len() == usize::from(*count) {
                    break 'repeat
                }
            },
            //TODO what if count is never received? 
            None => (),
        }
    }
}

pub fn recieve_master_results2(sock: &UdpSocket, addr_list: &mut Vec<Addr>, server_count: &mut Option<u16>) -> Result<(), Error> {
    loop {
        let mut buf: [u8; 1400] = [0; 1400];
        let buf_size: usize;
        match sock.recv_from(&mut buf) {
            Ok(x) => {
                buf_size = x.0;
            },
            Err(e) => return Err(e),
        }
        let buf: &[u8] = &buf[0..buf_size];
        let response = parse_response(&buf).unwrap();
        match response {
            Count(x) => *server_count = Some(x.0),
            List6(x) => {
                for addr in x.0.iter() {
                    addr_list.push(addr.unpack());
                }
            },
            _ => (),
        }
        match server_count {
            Some(count) => {
                if addr_list.len() == usize::from(*count) {
                    break;
                }
            }
            None => (),
        }
    }
    return Ok(());
}
        /* let (buf_size2, _addr) = sock.recv_from(&mut buf).unwrap();
        let buf_size = Some(buf_size2);
        match buf_size {
            Some(ref size) => {
                let buf: &[u8] = &buf[0..*size];
                let response = parse_response(&buf).unwrap();
                match response {
                    Count(x) => *server_count = Some(x.0),
                    List6(x) => {
                        for addr in x.0.iter() {
                            addr_list.push(*addr);
                        }
                    },
                    _ => Ok(),
                }
            },
            None => (),
        }
        //TODO remove
        //dbg!(&addr_list.len());
        //dbg!(&server_count);
        match server_count {
            Some(ref count) => {
                if addr_list.len() == usize::from(*count) {
                    break 'repeat
                }
            },
            //TODO what if count is never received? 
            None => (),
        } */

pub async fn send_recieve_masters(addr: &str, addr_list: &mut Vec<Addr6Packed>){
    let socket = UdpSocketTokio::bind("0.0.0.0:0").await.expect("could not bind socket");
    let mut vec: Vec<Addr6Packed> = vec![];

    let mut server_count: Option<u16> = Some(0);

    send_master_request(&socket, addr).await;
    recieve_master_results(&socket, & mut vec, &mut server_count).await;
    dbg!("out of send recieve");
    addr_list.append(&mut vec);
}

pub fn send_recieve_masters2(addr: &str, addr_list: &mut Vec<Addr>){
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");
    let mut vec: Vec<Addr> = vec![];

    let mut server_count: Option<u16> = Some(0);

    send_master_request2(&socket, addr);
    match recieve_master_results2(&socket, & mut vec, &mut server_count) {
        Ok(_) => {dbg!("Recieved all servers");},
        Err(_) => {dbg!("Did not recieve all servers");},
    }
    dbg!("out of send recieve");
    addr_list.append(&mut vec);
}

pub async fn send_info6_ex_request(sock: &UdpSocketTokio, addr: &str, challenge: u32) -> Option<usize> {
    let header_info_6 = request_info_6_ex(challenge);
    //sock.connect(addr).await.ok();
    let sent = sock.send_to(&header_info_6, addr).await.ok();
    /* match sent {
        Some(x) => println!("SENT {}", x),
        None => println!("SENTFAIL"),
    } */
    return sent;
}

pub fn send_info6_ex_request2(sock: &std::net::UdpSocket, addr: &str, challenge: u32) -> Option<usize> {
    let header_info_6 = request_info_6_ex(challenge);
    //sock.connect(addr).await.ok();
    let sent = sock.send_to(&header_info_6, addr).ok();
    /* match sent {
        Some(x) => println!("SENT {}", x),
        None => println!("SENTFAIL"),
    } */
    return sent;
}

pub async fn recieve_info_result(sock: &UdpSocketTokio) -> Option<ServerInfo> {
    let mut buf: [u8; 65507] = [0; 65507];
    let (buf_size2, _addr) = sock.recv_from(&mut buf).await.unwrap();
    let buf_size = Some(buf_size2);

    match buf_size {
        Some(ref size) => {
            let buf: &[u8] = &buf[0..*size];
            let partial_server_info: Option<PartialServerInfo>;
            let server_info: Option<ServerInfo> = loop {
                match parse_response(&buf).unwrap() {
                    Info6(x) => break Info6Response(x.0).parse(),
                    Info6Ex(x) => partial_server_info = Info6ExResponse(x.0).parse(),
                    Info6ExMore(x) => partial_server_info = Info6ExMoreResponse(x.0).parse(),
                    _ => break None,
                }
                match partial_server_info {
                    Some(mut part_info) => {
                        match part_info.get_info() {
                            Some(serv_info) => break Some(serv_info.clone()),
                            None => {
                                //TODO what to do when more data needs reading
                                //println!("Cant get for {}", _addr);
                                break None
                            },
                        }
                    }
                    None => break None,
                }
            };
            return server_info;
        },
        None => None,
    }
}

pub fn recieve_info_result3(
    sock: &std::net::UdpSocket, 
    server_infos: &mut HashMap<Addr, ServerInfo>,
    partial_server_infos: &mut HashMap<Addr, Vec<PartialServerInfo>>
) -> Result<(), Error> {

    let mut buf: [u8; 65507] = [0; 65507];
    let buf_size: usize;
    let addr: SocketAddr;
    match sock.recv_from(&mut buf) {
        Ok(x) => {
            buf_size = x.0;
            addr = x.1;
        }
        Err(e) => return Err(e),
    }

    let address = match addr.ip() {
        V4(x) => Addr { ip_address: IpAddr::V4(x), port: addr.port()} ,
        V6(x) => Addr { ip_address: IpAddr::V6(x), port: addr.port()},
    };

    let buf: &[u8] = &buf[0..buf_size];
    match parse_response(&buf[0..buf_size]).unwrap() {
        Info6(x) => {
            match Info6Response(x.0).parse() {
                Some(info) => {server_infos.insert(address, info);},
                None => (),
            }
        },
        Info6Ex(x) => {
            match Info6ExResponse(x.0).parse() {
                Some(partial_info) => {partial_server_infos.entry(address).or_default().push(partial_info);},
                None => (),
            }
        },
        Info6ExMore(x) => {
            match Info6ExMoreResponse(x.0).parse() {
                Some(partial_info) => {partial_server_infos.entry(address).or_default().push(partial_info);},
                None => (),
            }
        },
        Info664(_) => {
            dbg!("Got Info664");
        },
        _ => (),
    }

    return Ok(());
}


pub async fn send_recieve_server(challenge: u32, addr: String, timeout_millis: u64) -> Option<ServerInfo> {
    println!("Sent");
    let sock = UdpSocketTokio::bind("0.0.0.0:0").await.expect("Error");

    send_info6_ex_request(&sock, &addr, challenge).await;

    match timeout(Duration::from_millis(timeout_millis), recieve_info_result(&sock)).await {
        Ok(r) => {
            println!("Got");
            return r
        },
        Err(_) => {
            println!("Timeout");
            return None
        },
    };
}

pub async fn fetch_friend_data(online_friends: &mut Vec<String>, settings_path: String) {
    let mut addr_list: Vec<Addr6Packed> = vec![];

    send_recieve_masters("master4.teeworlds.com:8300", &mut addr_list).await;
    send_recieve_masters("master3.teeworlds.com:8300", &mut addr_list).await;

    dbg!(addr_list.len());

    let mut handles: Vec<tokio::task::JoinHandle<Option<ServerInfo>>> = vec![];
    let mut results: Vec<Result<Option<ServerInfo>, tokio::task::JoinError>> = vec![];

    let mut i = 0;
    for addr in addr_list.as_slice() {
        let handle: tokio::task::JoinHandle<Option<ServerInfo>> = tokio::spawn(send_recieve_server(i, addr.unpack().to_string(), 1000));
        handles.push(handle);
        i += 1;
        if i % 80 == 0 || usize::try_from(i).unwrap() == addr_list.len() {
            results.append(&mut futures::future::join_all(&mut handles).await);
            handles.clear();
        }
    }

    dbg!(results.len());

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

fn parse_partial_infos(partial_infos: & Vec<PartialServerInfo>) -> Option<ServerInfo> {
    if partial_infos.len() == 0 {
        return None;
    }
    let mut info = partial_infos[0].clone();
    for partial_info in &partial_infos[1..partial_infos.len()] {
        info.merge(partial_info.clone()).unwrap();
    }
    info.take_info()
}


pub fn fetch_friend_data_smart(_online_friends: &mut Vec<String>, _settings_path: String) -> Result<(), Error> {
    let mut addr_list: Vec<Addr> = vec![];

    send_recieve_masters2("master4.teeworlds.com:8300", &mut addr_list);
    send_recieve_masters2("master3.teeworlds.com:8300", &mut addr_list);

    dbg!(addr_list.len());
    //100, 200  100, 150?
    const CHUNK_SIZE: usize = 100;
    const TIMEOUT: u64 = 300;

    dbg!(CHUNK_SIZE, TIMEOUT);

    let mut server_infos: HashMap<Addr, ServerInfo> = HashMap::new();
    let mut partial_server_infos: HashMap<Addr, Vec<PartialServerInfo>> = HashMap::new();

    let mut chall = 0;
    let sock = UdpSocket::bind("0.0.0.0:0").expect("big nonono ooof");

    for chunk in addr_list.chunks(CHUNK_SIZE) {
        //TODO Spawn taks to do this part
        sock.set_read_timeout(Some(Duration::from_millis(TIMEOUT))).expect("msg");
        //sock.set_nonblocking(true).expect("EH");
        for addr in chunk {
            send_info6_ex_request2(&sock, &addr.to_string(), chall);
            chall += 1;
            println!("Sent")
        }
        //sock.set_nonblocking(false).expect("EH");
        let mut amount = 0;
        loop {
            match recieve_info_result3(&sock, &mut server_infos, &mut partial_server_infos) {
                Ok(_) => (),
                Err(_) => break,
            }
            amount += 1;
        }
        dbg!(amount);
    }

    let server_info_length = server_infos.len();
    let partial_server_info_length = partial_server_infos.len();

    //dbg!(server_infos);
    //dbg!(partial_server_infos);
    dbg!(server_info_length);
    dbg!(partial_server_info_length);

    for (addr, partial_infos) in partial_server_infos {
        if partial_infos.len() > 1 {
            dbg!(partial_infos.len());
        }
        match parse_partial_infos(&partial_infos) {
            Some(info) => {server_infos.insert(addr, info);},
            None => (),
        }

    }

    let server_info_length = server_infos.len();

    dbg!(server_info_length);
    dbg!(partial_server_info_length);

    Ok(())
}