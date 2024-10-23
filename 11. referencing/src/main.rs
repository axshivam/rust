fn main() {
    println!("Referencing");

    fun_one();
    fun_two();

    let mut str1: String = String::from("Shiv Sharma");
    fun_three(&mut str1);

    fun_four();

}

fn fun_four() {
    let mut str: String = String::from("Uttar Pradesh");

    let m1 = &mut str;
    m1.push_str(" India");
    let m2 = &str;
    println!("M2: {m2}");

    // Now it will give error
    // println!("m1: {m1}"); 
}

fn fun_three(str: &mut String) {
    str.push_str(" From Kanpur");
    println!("{str}");
}

fn fun_two() {
    let mut s1: String = String::from("Hello");

    let s2 = &mut s1;
    s2.push_str(" Shiv");
    println!("S2: {s2}");
    let s3 = &mut s1;
    s3.push_str(" Sharma");
    println!("S3: {s3}");
    // println!("S2: {s2} and S3: {s3}"); // This will give error, as mutable reference can be paased to only one 
}
fn fun_one() {
    let s1: String = String::from("Hello");

    let s2 = &s1;
    let s3 = &s1;
    println!("S2: {s2} and S3: {s3}");
}

