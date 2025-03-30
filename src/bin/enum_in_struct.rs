#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
fn route(ip_kind: &IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("route to ipv4"),
        IpAddrKind::V6 => println!("toute to ipv6"),
    }
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(&four);
    route(&six);

    println!("{:?}", four);
    println!("{:?}", six);
    println!("four is : {:#?}", four);
    
    let ip_addr1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("1234"),
    };

    println!("ip_addr1 is : {:#?}", ip_addr1);
} 