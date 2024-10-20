fn main() {
    println!("Variables");

    let a = 34;

    println!("A is {a}");

    let a = 32;

    println!("A is {a}");

    let mut a = 32;

    println!("A is {a}");
    a = 37;
    println!("A is {a}");

    shadowing();
}

fn shadowing() {
    let x = 4;
    println!("X1 is {x}");

    let x = x+1;
    println!("X2 is {x}");

    {
        let x = x*3;
        println!("X3 is {x}");
    }

    println!("X4 is {x}");
}
