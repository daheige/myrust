#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("v6 = {:?}", six);
    let ip = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    println!("ip type: {:?},address: {}", ip.kind, ip.address);
}
