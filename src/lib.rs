use std::{collections::HashMap, io, net::IpAddr};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum PortState {
    Open,
    Filtered,
    Closed,
}

pub async fn connect_scan(ip: IpAddr) -> Result<HashMap<u16, PortState>, io::Error> {
    let mut scan_data = HashMap::new();

    for port in std::u16::MIN..std::u16::MAX {
        if let Ok(_v) = TcpStream::connect((ip, port)).await {
            scan_data.insert(port, PortState::Open);
        }
    }
    Ok(scan_data)
}
