enum IpAddr{
    V4(String),
    V6(String)
}

enum Message{
    Quit,
    Move {x : i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        println!("call");
    }
}

enum Coin{
    Penny, 
    Nickel, 
    Dime,
    Quarter 
}

fn value_in_cents(coin: Coin) -> u8{
    let value = match coin{
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("This is a nickel");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    value
}

fn main() {
    let home =IpAddr::V4(String::from("127.0.0.1"));
    
    let message = Message::Write(String::from("test"));

    message.call();

    let coin:Coin = Coin::Nickel;
    
    println!("{}",value_in_cents(coin));
}
