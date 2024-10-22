fn main() {
    println!("Hello, world!");

    if 5 > 7 {
        println!("5 is greater than 7");
    } else {
        println!("5 is less than 7");
    }

    let k = greatest(345, 678, 111);
    println!("Greates : {k}");

    only_if();
}

fn greatest(a: i128, b: i128, c: i128) -> i128 {
    if a > b && a > c {
        a
    } else if b > a && b > c {
        b
    } else {
        c
    }
}

fn only_if() {
    if 6 < 9 {
        if 6 > 1 {
            println!("Only If");
        }
    }
}
