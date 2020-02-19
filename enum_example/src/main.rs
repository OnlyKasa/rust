enum IpAddrKind {
    V4,
    V6,
}


enum IpAddr {
    V4(String),
    V6(String),
}


struct IpAddr {
    kind: IpAddrKind, 
    address: String,
}

fn main() {
   
    let v4 = IpAddrKind::V4;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    }
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


}
