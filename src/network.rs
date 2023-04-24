use serverbrowse::protocol::*;
use serverbrowse::protocol::Response::*;
use std::time::Duration;
use tokio::time::timeout;
use tokio::net::UdpSocket;

pub async fn send_master_request(sock: &UdpSocket, addr: &str) {
    let header_count_6: [u8; 14] = request_count();
    let header_list_6 = request_list_6();
    //sock.connect(addr).await.ok();
    sock.send_to(&header_count_6, addr).await.ok();
    sock.send_to(&header_list_6, addr).await.ok();
}

pub async fn recieve_master_results(sock: &UdpSocket, addr_list: &mut Vec<Addr6Packed>, server_count: &mut Option<u16>) {
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

pub async fn send_recieve_masters(addr: &str, addr_list: &mut Vec<Addr6Packed>){
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("could not bind socket");
    let mut vec: Vec<Addr6Packed> = vec![];

    let mut server_count: Option<u16> = Some(0);

    send_master_request(&socket, addr).await;
    recieve_master_results(&socket, & mut vec, &mut server_count).await;
    addr_list.append(&mut vec);
}

pub async fn send_info6_ex_request(sock: &UdpSocket, addr: &str, challenge: u32) -> Option<usize> {
    let header_info_6 = request_info_6_ex(challenge);
    //sock.connect(addr).await.ok();
    let sent = sock.send_to(&header_info_6, addr).await.ok();
    /* match sent {
        Some(x) => println!("SENT {}", x),
        None => println!("SENTFAIL"),
    } */
    return sent;
}

pub async fn recieve_info_result(sock: &UdpSocket) -> Option<ServerInfo> {
    let mut buf: [u8; 4096] = [0; 4096];
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

pub async fn send_recieve_server(challenge: u32, addr: String, timeout_millis: u64) -> Option<ServerInfo> {
    //println!("START");
    let sock = UdpSocket::bind("0.0.0.0:0").await.expect("Error");

    send_info6_ex_request(&sock, &addr, challenge).await;

    match timeout(Duration::from_millis(timeout_millis), recieve_info_result(&sock)).await {
        Ok(r) => {
            //println!("DONE");
            return r
        },
        Err(_) => {
            //println!("TIMEOUT");
            return None
        },
    };
}