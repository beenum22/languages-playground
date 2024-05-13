enum IpAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

impl IpAddr {
    fn pretty_print(&self) -> String {
        println!()
    }
}

// struct IpAddr {
//     kind: IpAddrType,
//     address: String,
// }

fn main() {
    let loopback_v4 = IpAddr::IPv4(127, 0, 0, 1);

    let loopback_v6 = IpAddr::IPv6(String::from("::1"));
}
