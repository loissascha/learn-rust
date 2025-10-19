enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Lets get Rusty!");
    }
}

fn main() {
    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    Message::some_function();

    // optional enums -> included in base language as it's used for null/none values
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let x = Some(5);
    let y = 3;

    let sum = x.unwrap_or(0) + y;

    value_in_cents(Coin::Quarter(UsState::Arizona));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    plus_one(Some(3));
}

// match with Optional values (either none or some)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(3) => {
            println!("three");
            Some(4)
        }
        Some(i) => Some(i + 1),
        _ => None, // alternative to "None" in this case .. or basically the default
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// match with enum
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn route(ip_kind: IpAddrKind) {}
