fn main() {
    println!("Hello, world!");

    simple_loop();
    while_loop();
    let sum = for_loop(124, 128);
    println!("Sum is : {sum}");
}

fn simple_loop() {
    let mut i = 0;
    loop {
        println!("{i} : {}", i * 2);
        i = i + 1;
        if i == 10 {
            break;
        }
    }
}

fn while_loop() {
    let mut a = 10;

    while a > 0 {
        println!("a : {a}");
        a -= 1;
    }
}

fn for_loop(a: i16, b: i16) -> i16 {
    let mut count: i16 = 0;
    for i in a..b {
        count += i;
    }

    count
}
