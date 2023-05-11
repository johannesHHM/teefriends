
use serverbrowse::protocol::*;
use serverbrowse::protocol::Response::*;
use std::time::Duration;
use std::io::Error;

use std::net::IpAddr::{V4, V6};
use std::net::{UdpSocket, SocketAddr};

use std::collections::HashMap;

use crate::settings::read_friends;

fn send_master_request(sock: &UdpSocket, addr: &str) {
    let header_count_6: [u8; 14] = request_count();
    let header_list_6 = request_list_6();
    sock.send_to(&header_count_6, addr).ok();
    sock.send_to(&header_list_6, addr).ok();
}

fn recieve_master_results(sock: &UdpSocket, addr_list: &mut Vec<Addr>, server_count: &mut Option<u16>) -> Result<(), Error> {
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

fn send_recieve_masters(addr: &str, addr_list: &mut Vec<Addr>) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");
    socket.set_read_timeout(Some(Duration::from_millis(2000))).unwrap();
    let mut vec: Vec<Addr> = vec![];

    let mut server_count: Option<u16> = Some(0);

    send_master_request(&socket, addr);
    match recieve_master_results(&socket, & mut vec, &mut server_count) {
        Ok(_) => (), //{dbg!("Recieved all servers");},
        Err(_) => (), //{dbg!("Did not recieve all servers");},
    }
    //dbg!("out of send recieve");
    addr_list.append(&mut vec);
}

fn send_info6_ex_request(sock: &std::net::UdpSocket, addr: &str, challenge: u32) -> Option<usize> {
    let header_info_6 = request_info_6_ex(challenge);
    let sent = sock.send_to(&header_info_6, addr).ok();
    return sent;
}

fn recieve_info_result(
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
            //dbg!("Got Info664");
        },
        _ => (),
    }

    return Ok(());
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

pub fn fetch_friend_data(online_friends: &mut Vec<String>, settings_path: &String) -> Result<(), Error> {
    let mut addr_list: Vec<Addr> = vec![];

    send_recieve_masters("master4.teeworlds.com:8300", &mut addr_list);
    send_recieve_masters("master3.teeworlds.com:8300", &mut addr_list);

    //dbg!(addr_list.len());
    //100, 200  100, 150?
    const CHUNK_SIZE: usize = 100;
    const TIMEOUT: u64 = 250;

    //dbg!(CHUNK_SIZE, TIMEOUT);

    let mut server_infos: HashMap<Addr, ServerInfo> = HashMap::new();
    let mut partial_server_infos: HashMap<Addr, Vec<PartialServerInfo>> = HashMap::new();

    let mut chall = 0;
    let sock = UdpSocket::bind("0.0.0.0:0").expect("big nonono ooof");

    for chunk in addr_list.chunks(CHUNK_SIZE) {
        sock.set_read_timeout(Some(Duration::from_millis(TIMEOUT))).expect("msg");
        //sock.set_nonblocking(true).expect("EH");
        for addr in chunk {
            send_info6_ex_request(&sock, &addr.to_string(), chall);
            chall += 1;
        }
        //sock.set_nonblocking(false).expect("EH");
        let mut _amount = 0;
        loop {
            match recieve_info_result(&sock, &mut server_infos, &mut partial_server_infos) {
                Ok(_) => (),
                Err(_) => break,
            }
            _amount += 1;
        }
        //dbg!(_amount);
    }

    let _server_info_length = server_infos.len();
    let _partial_server_info_length = partial_server_infos.len();

    //dbg!(_server_info_length);
    //dbg!(_partial_server_info_length);

    for (addr, partial_infos) in partial_server_infos {
        //if partial_infos.len() > 1 {
        //    dbg!(partial_infos.len());
        //}
        match parse_partial_infos(&partial_infos) {
            Some(info) => {server_infos.insert(addr, info);},
            None => (),
        }
    }

    let _server_info_length = server_infos.len();

    //dbg!(_server_info_length);
    //dbg!(_partial_server_info_length);

    let friends = read_friends(settings_path)?;

    for server_info in server_infos.values() {
        for friend in &friends {
            for client in &server_info.clients {
                if *friend == client.name.to_string() {
                    online_friends.push(client.name.to_string());
                }
            }
        }
    }

    Ok(())
}