enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    
}

fn main() {
    println!("Hello, world!");
    let op: Option<u8> = Option::Some(1);
    let x = 7;

    // op + x // Not possible
    let op1: Option<u8> = Some(1);
    let none: Option<i32> = Option::None;
    let m: Message = Message::Write(String::from("SomeThing"));

    let a = Some(1).is_some();
    println!("a: {a}");

    let a1 = Some(1);
    let x = 32;
    let sum = x + a1.unwrap();
    println!("Sum: {sum}");

    // we can not use unwrap on None
}
