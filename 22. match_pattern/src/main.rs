#[warn(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Custom(u8)
}
enum UsState {

}

fn main() {
    let coin_1: Coin = Coin::Penny;
    let res = value_in_cent(coin_1);
    println!("Res = {res}");

    let coin_2: Coin = Coin::Custom(8);
    let res2 = value_in_cent(coin_2);
    println!("Res 2 = {res2}");

    println!("Add result: {}", add(90, Some(80)));
    println!("Add result  for none: {}", add(90, None));
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Custom(value) => {
            value
        }
    }
}

fn add(num1: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => {
            num1 + i
        },
        None => {
            0
        }
        // _ and other are same, but other also give variable name
    }
}