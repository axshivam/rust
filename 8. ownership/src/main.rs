fn main() {
    println!("Hello, world!");

    let str : &str = "Hi My Name is Shiv";
    println!("{str}");

    let mut str1 : &str = "New";
    println!("{str1}");

    str1 = "Shiv Sharma";
    println!("{str1}");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // When a variable goes out of scope, Rust calls a special function for us.
    // This function is called drop, and it’s where the author of String can put the code to return the memory.
    // Rust calls drop automatically at the closing curly bracket.

    let s1 = 5;
    let s2 = s1;

    println!("Values: {s1} & {s2}");

    let s3 = "Hello";
    let s4 = s3;
    println!("Values: {s3} & {s4}");

    let s5 = String::from("My name Something");
    let s6 = s5;
    println!("V123 : & {s6}");

    let s7 = s6.clone();
    println!("s6 : {s6} and s7 : {s7}");
}
