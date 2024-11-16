fn main() {
    let a = 21;
    let b = 22;

    let ans = main_1(a,b);

    println!("Sum of these two numbers are: {}", ans);
}


fn main_1(x: i32,y: i32) -> i32 {
    return x+y;
}
