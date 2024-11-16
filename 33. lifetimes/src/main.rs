fn main() {
    dangling_references();

    fixed_dangling_reference();

    // Generic Lifetimes in Functions
    let str1 = String::from("Shiv");
    let str2 = "Shiv Sharma";
    let result = longest(str1.as_str(), str2);

    println!("Longest string is : {result}");
}


// Preventing Dangling References with Lifetimes
fn dangling_references() {
    let result;

    // println!("Result before initializing: {result}");
    {
        let x = 5;
        result = &x;

        println!("Result is : {result}");
    }

    // println!("Result outside: {result}"); // borrowed value does not live long enough
}

fn fixed_dangling_reference() {
    let x = 5;
    let res = &x;

    println!("Res is: {res}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}


