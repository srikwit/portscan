use std::net::{IpAddr,TcpStream};
use std::collections::HashMap;

#[derive(Debug)]
pub enum PortState{
Open,
Filtered,
Closed
}


pub fn port_scan(ip: IpAddr) -> HashMap<u16,PortState> {
 let mut scan_data = HashMap::new();
 for port in std::u16::MIN .. std::u16::MAX {
     if let Ok(_stream) = TcpStream::connect((ip,port)) {
         scan_data.insert(port,PortState::Open);
        }
     else{
         continue;
        }
    }
    scan_data
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
