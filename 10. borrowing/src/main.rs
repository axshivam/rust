fn main() {
    println!("Borrowing in Ownership");

    let str: String = String::from("Hello");

    let length: usize = string_length(str);

    println!("Length of is {length}");
    // println!("Length of {str} is {length}"); // gives error as str is null now

    let str1: String = String::from("Kanpur");

    let (len, str) : (usize, String) = len(str1);

    println!("Length of {str} is equal to {len}");

    let str2: String = String::from("Uttar Pradesh");

    let length1: usize = solution(str2.clone());

    println!("3 -> Length of {str2} is {length1}");

    let final_string: String = String::from("Shiv Sharma");

    let size: usize = final_solution(&final_string);

    println!("Final string is {final_string} and length is {size}");
}


fn string_length(s: String) -> usize {
    return s.len();
}

fn len(str: String) -> (usize, String) {
    return (str.len(), str);
}

fn solution(str: String) -> usize {
    return str.len();
}

fn final_solution(str: &String) -> usize {
    return str.len();
}
