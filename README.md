# portscan
A library to provide port scanning functionality.

Current scanning features:

[✓] Connect scan 
```
Best performance:

Nmap:
time nmap -sT -p 0-65535 127.0.0.1
real    0m1.303s
user    0m0.386s
sys     0m0.899s

portscan:
time cargo run --release
real    0m2.024s
user    0m0.703s
sys     0m1.691s
```

[X] UDP scan

[X] SYN scan

[X] ACK scan

[X] Window scan

[X] Maimon scan

[X] Null scan

[X] FIN scan

[X] XMAS scan

[X] Idle scan

[X] FTP bounce scan


Current language features:

[✓] Nightly build

[✓] Clippy

[✓] Async/IO

[X] Multi-threading

[X] Packet sniffing

[X] Testing

Usage:
```
use std::net::{IpAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}",portscan::connect_scan("127.0.0.1".parse::<IpAddr>()?).await?);
        Ok(())
}
```
