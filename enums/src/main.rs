fn main() {
    let four = IpAddrKind::V4(String::from("192.168.0.1"));
    let six = IpAddrKind::V6(String::from("127.0.0.1"));
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

