use serverbrowse::protocol::*;
use serverbrowse::protocol::Response::*;
use std::time::Duration;

use std::net::UdpSocket;
use std::vec;

fn send_master_request(sock: &UdpSocket, addr: &str) {
    let header_count_6: [u8; 14] = request_count();
    let header_list_6 = request_list_6();
    sock.connect(addr).ok();
    sock.send(&header_count_6).ok();
    sock.send(&header_list_6).ok();
}

fn recieve_master_results(sock: &UdpSocket, addr_list: &mut Vec<Addr6Packed>, server_count: &mut Option<u16>) {
    'repeat: loop {
        let mut buf: [u8; 1400] = [0; 1400];
        let buf_size = sock.recv(&mut buf).ok();

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

fn send_info6_request(sock: &UdpSocket, addr: &str) -> Option<usize> {
    let header_info_6 = request_info_6(0);
    sock.connect(addr).ok();
    let sent = sock.send(&header_info_6).ok();
    return sent;
}

fn recieve_info_result(sock: &UdpSocket) -> Option<ServerInfo> {
    let mut buf: [u8; 4096] = [0; 4096];
    let buf_size = sock.recv(&mut buf).ok();
    match buf_size {
        Some(ref size) => {
            let buf: &[u8] = &buf[0..*size];
            let response = parse_response(&buf).unwrap();

            match response {
                Info6(x) => Info6Response(x.0).parse(),
                _ => None,
            }
        },
        None => None,
    }
}

fn main() {
    let timeout = Duration::from_millis(1000);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");
    socket.set_read_timeout(Some(timeout)).ok();
    socket.set_write_timeout(Some(timeout)).ok();

    send_master_request(&socket, "master3.teeworlds.com:8300");

    let mut addr_list: Vec<Addr6Packed> = vec![];
    let mut server_count: Option<u16> = None;

    recieve_master_results(&socket, &mut addr_list, &mut server_count);

    for addr in addr_list.as_slice() {
        println!("{}", addr.unpack());
        
        send_info6_request(&socket, &addr.unpack().to_string());
        let res = recieve_info_result(&socket);

        match res {
            Some(ref r) => println!("{:?}", r),
            None => println!("Result returned None"),
        }
    }
}
