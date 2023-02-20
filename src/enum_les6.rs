pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
/*struct IpAddr {
    kind : IpAddrKind,
    address : String
}*/
pub fn enum_with_construct() {
    /* let localhost = IpAddr{
         kind: IpAddrKind::V4,
         address: "127.0.0.1".to_string(),
     };
     println!(" address={}", localhost.address);*/

    //let localHost = IpAddr::V4(127, 0, 0, 1);
}


pub fn if_let_test() {
    let message = Message::ChangeColor(1, 2, 3);

    if let Message::ChangeColor(a,_,_) = message {
        println!("Max {}", a);
    }
}

pub fn matcher() -> i32 {
    let message = Message::ChangeColor(1, 2, 3);
    match message {
        Message::Quit => {
            println!("Exit program");
            1
        }
        Message::Move { .. } => { 2 }
        Message::Write(_) => { 3 }
        _ => {4}
    }
}


pub fn test_option() -> u32 {
    let a: u32 = 444;
    //let option_null = None;
    let option_b = Some(444);
    //bad operations:
    //option_null == option_b;
    //a + option_b == 888

    //match:
    match option_b {
        None => { false }
        Some(_) => { true }
    };

    option_b.unwrap()
}