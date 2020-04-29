#![crate_name = "portscan"]
use std::{collections::HashMap, io, net::IpAddr};
use tokio::net::TcpStream;

/// Represents a port state. A port can either be open, closed or filtered.
#[derive(Debug)]
pub enum PortState {
/// An open port indicates that the client is able to send and receive data to the server.
    Open,
/// A closed port indicates that the client is unable to receive data from the server.
    Filtered,
/// A filtered port is the inability of the application to understand if the port is open or closed.
    Closed,
}

/// Returns a HashMap containing a mapping between a port and its state.
///
/// # Arguments
///
/// * `ip` - A valid IP address which will be probed against all of its ports
///
/// # TODO
/// * Change returnt type from HashMap to a list as only open connections are being returned so there is only one state.
/// This should change the port states to just open and filtered.
/// I am not yet sure, why a closed port state is therefore required but I am retaining it for safe similarity with Nmap.
/// 
/// ```
/// use portscan;
/// portscan::connect_scan("127.0.0.1".parse::<IpAddr>()?).await?);
///
/// ```
pub async fn connect_scan(ip: IpAddr) -> Result<HashMap<u16, PortState>, io::Error> {
    let mut scan_data = HashMap::new();

    for port in std::u16::MIN..std::u16::MAX {
        if let Ok(_v) = TcpStream::connect((ip, port)).await {
            scan_data.insert(port, PortState::Open);
        }
    }
    Ok(scan_data)
}
