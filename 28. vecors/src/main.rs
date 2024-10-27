fn main() {
    let vec1 : Vec<u32> = Vec::new();
    println!("vec1: {vec1:?}");

    let vec2 = vec![12,23,34,45];
    println!("vec2: {vec2:?}");

    let mut vec3 : Vec<String> = Vec::new();
    vec3.push(String::from("Shiv"));
    vec3.push(String::from("Sharma"));
    let size3 = vec3.len();
    println!("vec3: {vec3:?} and size of vec3 is {size3}");

    let mut vec4 = vec![1,2,3,4,5,6,7,8];
    vec4[0] = 12;
    for element in &mut vec4 {
        println!("Elements: {element}");
        *element = *element + 4; // gives error
    }
    println!("vec4: {vec4:?}");

    
}
