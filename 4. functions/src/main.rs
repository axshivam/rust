fn main() {
    println!("Hello, world!");

    sec_fun();
    third_fun(345);
    fourth_fun(6543, 'K');

    let y: i128 = return_value();

    println!("Y is {y}");

    exp();
}

fn sec_fun() {
    println!("Second Function");
}

fn third_fun(x: i32) {
    println!("X is: {x}");
}

fn fourth_fun(x: i128, p: char) {
    println!("X is {x} and P is {p}");
}

fn return_value() -> i128 {
    3450 + 23456
}

fn exp() {
    let y = {
        let x = 9;
        x + 1
    };

    println!("The value of y is {}", y);
}
