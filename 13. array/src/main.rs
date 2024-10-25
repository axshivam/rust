fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    println!("arr : {:?}", arr);
    let arr: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("arr : {:?}", arr);

    let mut arr: [i32; 4] = [1, 2, 3, 4];
    arr[0] = 12;
    println!("arr : {:?}", arr);
    println!("Size of array: {}", arr.len());

    let sum: i32 = sum_of_array(arr);

    println!("Sum : {sum}");

    // pass a mutable array
    let arr1 = [23, 12, 34, 234, 789];

    mutable_array(arr1);
    println!("Arr1 : {:?}", arr1);

    // pass the reference of array
    let mut arr2 : [u32; 5] = [100,200,300,400,500];
    arr2[2] = 121;
    println!("Arr2 : {:?}", arr2);

    reference_change(&mut arr2);

    println!("Arr2 : {:?}", arr2);
}

fn reference_change(arr2: &mut [u32; 5]) {

    arr2[3] = 1111;
}



fn mutable_array(mut arr1: [i128; 5]) {
    arr1[0] = 146;

    println!("Arr1: {:?}", arr1);
}

fn sum_of_array(p: [i32; 4]) -> i32 {
    let length = p.len();

    let mut i = 0;
    let mut sum = 0;
    while i < length {
        sum += p[i];
        i += 1;
    }

    return sum;
}
