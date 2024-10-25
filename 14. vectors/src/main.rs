fn main() {
    println!("Vectors");

    let mut vec: Vec<i32> = Vec::new();
    vec.push(10);
    vec.push(11);
    println!("Vec: {:?}", vec);
    println!("Length : {}", vec.len());
    vec.pop();

    println!("Vec: {:?}", vec);
    println!("Length : {}", vec.len());

    let vec = Vec::<i32>::new();
    println!("Vec: {:?}", vec);
    println!("Length : {}", vec.len());

    let mut vec = vec![12,23,123,1234];
    vec.push(12345);

    println!("Vec: {:?}", vec);
    println!("Length : {}", vec.len());

    let mut str: Vec<&str> = vec!["Shiv", "Sharma", "from"];
    str.push("Kanpur");
    read_vec(&mut str);
    println!("Str: {:?}", str);

    write_fun(&mut str);
    println!("Str: {:?}", str);
}

fn write_fun(str: &mut Vec<&str>) {
    str.push("1");
    println!("Str from write function -> {:?}", str);
}

fn read_vec(str: &mut Vec<&str>) {
    println!("Str from function -> {:?}", str);
}
