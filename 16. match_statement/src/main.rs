fn main() {
    fun_one();

    fun_two();
}

fn fun_two() {
    let a = 10;

    match a {
        5 | 3 => println!("Number is 5 or 3"),
        10 | 1 => println!("Number is 1 or 10"),
        _ => println!("Default case"),
    }
}

fn fun_one() {
    let number = 5;
    match number {
        1 => println!("Number is one"),
        2 => println!("Number is two"),
        3 => println!("Number is three"),
        4 => println!("Number is four"),
        5 => println!("Number is five"),
        _ => println!("Number not exist"),
    }
}
