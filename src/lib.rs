#![crate_name = "portscan"]
use std::net::IpAddr;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::net::UdpSocket;
use tokio::{io, time};

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

/// Returns a Vector containing open TCP ports of the probed server.
///
/// # Arguments
///
/// * `ip` - A valid IP address
///
/// * TODO
///
/// Evaluate use of port states
/// This should change the port states to just open and filtered.
/// I am not yet sure, why a closed port state is therefore required but I am retaining it for safe similarity with Nmap.
/// The data returned for the UDP scan is currently those ports which are able to return data back within an expected duration.
/// Better logic has to be implemented to fully determine an open port.
///
/// ```
/// use portscan;
/// portscan::connect_scan("127.0.0.1".parse::<IpAddr>()?).await?);
///
/// ```
pub async fn connect_scan(ip: IpAddr) -> Result<Vec<u16>, io::Error> {
    let mut scan_data = Vec::<u16>::new();

    for port in std::u16::MIN..std::u16::MAX {
        if let Ok(_v) = TcpStream::connect((ip, port)).await {
            scan_data.push(port);
        }
    }
    Ok(scan_data)
}

/// Returns a Vector containing open UDP ports of the probed server.       
///
/// # Arguments
///
/// * `ip` - A valid IP address
///
/// * TODO
///
/// Evaluate use of port states
/// This should change the port states to just open and filtered.
/// I am not yet sure, why a closed port state is therefore required but I am retaining it for safe similarity with Nmap.
///    
/// ```
/// use portscan;
/// portscan::udp_scan("127.0.0.1".parse::<IpAddr>()?).await?);
///    
///
pub async fn udp_scan(ip: IpAddr) -> Result<Vec<u16>, io::Error> {
    let mut scan_data = Vec::<u16>::new();
    let timeout = Duration::from_millis(1000);

    // We use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = match if ip.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" }.parse() {
        Ok(v) => v,
        Err(e) => panic!("Unable to assign port for the UDP listener. {}", e),
    };

    let mut socket = UdpSocket::bind(local_addr).await?;
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    let send_data = vec![65; 65];
    for port in std::u16::MIN..std::u16::MAX {
        if let Ok(_v) = socket.connect((ip, port)).await {
            let mut receive_data = vec![0u8; MAX_DATAGRAM_SIZE];
            socket.send(&send_data).await?;
            match time::timeout(timeout, socket.recv(&mut receive_data)).await {
                Ok(v) => match v {
                    Ok(len) => {
                        if len > 0 {
                            scan_data.push(port);
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                },
                Err(_) => {
                    continue;
                }
            }
        }
    }
    Ok(scan_data)
}
