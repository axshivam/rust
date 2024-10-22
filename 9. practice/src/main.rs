use std::io;

fn main() {
    println!("Enter a name:");
    let mut guess = String::new();
 
    // io::stdin().read_line(&mut guess).expect("failed to readline");
 
    // print!("You entered {}", guess);

    io::stdin().read_line(&mut guess).expect("Failed to readline");

    let n: i32 = guess.trim()  // Remove any whitespace
        .parse() // Try to convert the string to a number
        .expect("Please enter a valid number");

    sums(n);
}

fn sums(n: i32) {
    println!("1: {n}");
    let mut number = n;

    let mut count = 0;

    while number > 0 {
        count += number % 10;
        number /= 10;
    }

    println!("Sum of digits: {count}");
}
