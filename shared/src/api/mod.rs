use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const IP_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const PORT: u16 = 8000;

pub fn get_socket_address() -> SocketAddr {
    SocketAddr::new(IP_ADDRESS, PORT)
}

pub fn get_api_url() -> String {
    format!("http://{}:{}/", IP_ADDRESS, PORT)
}
