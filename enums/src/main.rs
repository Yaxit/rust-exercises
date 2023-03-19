enum IpAddrKind {
    V4,
    V6
}

enum IpAddr {
    V4(String),
    V6(String),
    V69(u64, String)
}

impl IpAddr {
    fn Display(&self) { 
        match self {
            IpAddr::V4(val) => println!("V4> {}", val),
            IpAddr::V6(val) => println!("V6> {}", val),
            IpAddr::V69(num, val) => println!("V69> lucky {} with {}", num, val),
            _ => (),
        }
    }
}

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("192.168.1.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    let verynice = IpAddr::V69(42, String::from("Fun"));

    home.Display();
    loopback.Display();
    verynice.Display();

    // if let useful if interested in only one option, ingnores the others
    if let IpAddr::V4(s) = home {
        println!("Extracted {} with if let", s);
    }
}
