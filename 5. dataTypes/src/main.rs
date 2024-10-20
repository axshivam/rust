fn main() {
    println!("Hello, world!");

    let i: i8 = -1;

    println!("i : {i}");

    let j: i128 = 2894758;

    println!("j : {j}");

    let x: char = '1';
    let y: &str = "a";
    let z = String::from("Shiv Sharma");
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");

    let abc = true;
    let acb = false;
    println!("1 -> {abc}");
    println!("2 -> {acb}");

    let float_value: f64 = 90.90;
    println!("Float Value: {float_value}");

    // array and tuple
    let tuple = (23, 456, 234);
    println!("Tuple : {:?}", tuple);
    println!("Value 1: {}", tuple.0);
    println!("Value 2: {}", tuple.1);

    let tuple1: (i8, u8, char) = (-2, 5, 'C');
    println!("Tuple : {:?}", tuple1);

    let arr = [23, 45, 1111];
    println!("Arr : {:?}", arr);

    println!("Value 1: {}", arr[0]);
    println!("Value 2: {}", arr[1]);
}
