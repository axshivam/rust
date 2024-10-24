fn main() {
    fun_one();
    fun_two();
    calculate_length();
    handle_dangling_reference();
}

fn handle_dangling_reference() {
    // let k = solution();

    // println!("Value of K is: {:p}", k);
}

// fn solution() -> &i32 {
//     let a:i32 = 100;

//     return &a; // it will give error, as lifetime of a is over here;
// }

fn calculate_length() {
    let a = String::from("Hi 0");
    let len = length(&a);
    println!("Length of a is: {len}");
}

fn length(p: &String) -> usize {
    // return (*p).len(); // just same working
    return p.len(); // auto referencing
}
fn fun_two() {
    let mut a = 6;
    let b = &mut a;
    // b = b+1; // error
    *b = *b+1;

    println!("Value of a: {a}");
}
fn fun_one() {
    let a = 5;
    let b = &a;

    println!("a = {a} and b = {b}"); // automatic derefencing

    let c = 10;
    let d = &c;
    println!("d: {d} and {:p}", &d);
    println!("d = {:p} and address of c: {:p}", d, &c);
}


