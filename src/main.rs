use serverbrowse::protocol::*;
use serverbrowse::protocol::Response::*;
use std::time::Duration;

use tokio::net::UdpSocket;
use std::vec;

async fn send_master_request(sock: &UdpSocket, addr: &str) {
    let header_count_6: [u8; 14] = request_count();
    let header_list_6 = request_list_6();
    sock.connect(addr).await.ok();
    sock.send(&header_count_6).await.ok();
    sock.send(&header_list_6).await.ok();
}

async fn recieve_master_results(sock: &UdpSocket, addr_list: &mut Vec<Addr6Packed>, server_count: &mut Option<u16>) {
    'repeat: loop {
        let mut buf: [u8; 1400] = [0; 1400];
        let buf_size = sock.recv(&mut buf).await.ok();

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

/* fn send_info6_ex_request(sock: &UdpSocket, addr: &str, challenge: u32) -> Option<usize> {
    let header_info_6 = request_info_6_ex(challenge);
    sock.connect(addr).ok();
    let sent = sock.send(&header_info_6).ok();
    return sent;
}

fn recieve_info_result(sock: &UdpSocket) -> Option<PartialServerInfo> {
    let mut buf: [u8; 4096] = [0; 4096];
    let buf_size = sock.recv(&mut buf).ok();
    match buf_size {
        Some(ref size) => {
            let buf: &[u8] = &buf[0..*size];
            let response = parse_response(&buf).unwrap();
            match response {
                Info6(x) => {
                    dbg!("Info6 response");
                    //Info6Response(x.0).parse()
                    None
                },
                Info664(x) => {
                    dbg!("Info6 64 response");
                    None
                },
                Info6Ex(x) => {
                    dbg!("Info6 ex response");
                    Info6ExResponse(x.0).parse()
                },
                Info6ExMore(x) => {
                    dbg!("Info6 ex more response");
                    Info6ExMoreResponse(x.0).parse()
                },
                _ => {
                    dbg!("Defaulted out");
                    None
                },
            }
        },
        None => None,
    }
} */
#[tokio::main]
async fn main() {
    //let timeout = Duration::from_millis(1000);
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("could not bind socket");
    //socket.set_read_timeout(Some(timeout)).ok();
    //socket.set_write_timeout(Some(timeout)).ok();

    send_master_request(&socket, "master4.teeworlds.com:8300").await;

    let mut addr_list: Vec<Addr6Packed> = vec![];
    let mut server_count: Option<u16> = None;

    recieve_master_results(&socket, &mut addr_list, &mut server_count).await;

    println!("{:?}", server_count);
    //TODO change to real
    //let mut partial_server_infos: Vec<Vec<PartialServerInfo>> = vec![Vec::new(); addr_list.len()];
    /* let mut partial_server_infos: Vec<Vec<PartialServerInfo>> = vec![Vec::new(); 100];

    let mut i = 0;
    for addr in addr_list.as_slice() {
        println!("{}", addr.unpack());

        //TODO remove
        if i == 100 {
            break
        }
        
        send_info6_ex_request(&socket, &addr.unpack().to_string(), i);
        let res = recieve_info_result(&socket);
        match res {
            Some(r) => {
                println!("{:?}", r);

                let token = usize::try_from(r.token());
                println!("i: {}, token: {}", i, token.ok().unwrap());

                match token {
                    Ok(x) => {
                        partial_server_infos.get_mut(x).unwrap().push(r);
                    },
                    Err(_) => println!("Error with token"),
                }
            },
            None => println!("Result returned None"),
        }
        i += 1;
        println!();
    }
    println!("Afterwords");
    recieve_info_result(&socket);
    recieve_info_result(&socket);
    recieve_info_result(&socket);
    recieve_info_result(&socket);

    for (i,v) in partial_server_infos.iter().enumerate() {
        println!("Index: {} Length: {}", i, v.len());
    } */
}
