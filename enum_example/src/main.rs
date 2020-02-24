
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind, 
    address: String,
}


fn main() {
   
    let v4 = IpAddrKind::V4;
    println!(".....{:?}", IpAddrKind::V4);
    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!(".....{:?}", loopback);

    let b : Option<String> = Some(String::from("s: &str"));

    let c = match b {
        Some(s) => {
           println!("{}", "some value!!!");
        } ,
        None => {
            println!("{}", "none value!!!");
        }
    };
    
    let op_none: Option<String> = None;
    let op : Option<u32> = Some(4);
    match op_none {
        Some(value) => {
            println!("some value = {}", value);
        }
        None => {
            println!("this is none")
        }
    }
}