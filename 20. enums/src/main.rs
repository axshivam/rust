#[derive(Debug)]
enum IPAddressKind {
    V4,
    V6,
}

struct IPAddress {
    address: String,
    kind: IPAddressKind,
}

impl IPAddress {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            kind: IPAddressKind::V4,
        }
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKindNew {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
struct Human {
    height: u16,
    weight: u16,
}
#[derive(Debug)]
struct Animal {
    danger: String,
    face: u16,
}

#[derive(Debug)]
enum LivesKind {
    H(Human),
    A(Animal),
}

fn main() {
    let four: IPAddressKind = IPAddressKind::V4;
    //    route("127.0.01", IPAddressKind::V4);
    // route("127.0.01", four);

    let google_address = IPAddress {
        address: String::from("127.0.0.1"),
        kind: IPAddressKind::V4,
    };

    route(&google_address);

    println!(
        "Routing request to IP {} of kind {:?}",
        google_address.address, google_address.kind
    );

    let facebook_address = IPAddress::new("1.2.3.4");
    route(&facebook_address);

    let home: IpAddrKind = IpAddrKind::V4(String::from("12.13.14.15"));

    let home_new: IpAddrKindNew = IpAddrKindNew::V4(127, 0, 0, 1);
    let loopback: IpAddrKindNew = IpAddrKindNew::V6(String::from("::1"));
    route_new(&home);
    route_1(&home_new);
    route_1(&loopback);

    let human_1: Human = Human {
        height: 32,
        weight: 56,
    };

    let animal_1: Animal = Animal {
        face: 21,
        danger: String::from("Very"),
    };

    let human: LivesKind = LivesKind::H(human_1);
    let animal: LivesKind = LivesKind::A(animal_1);

    earth(&human);
    earth(&animal);
}

fn earth(ip: &LivesKind) {
    // dbg!(ip);
    println!("LivesKind {:?}", ip);
}

fn route_new(ip: &IpAddrKind) {
    // dbg!(ip);
    println!("Routing request to {:?}", ip);
}

fn route_1(ip: &IpAddrKindNew) {
    // dbg!(ip);
    println!("Routing request to {:?}", ip);
}

// 127.0.0.1
// fn route(ip: &str, kind: IPAddressKind) {
//     println!("Routing request to IP {ip} of kind {kind:?}");
// }

fn route(ip: &IPAddress) {
    println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind);
}
