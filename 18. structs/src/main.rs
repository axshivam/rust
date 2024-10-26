#[warn(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Structs!");

    // let tuple = (1,2, "String");
    // println!("{:?}", tuple);

    let user1: User = User {
        email: String::from("shivsharmavictory@gmail.com"),
        username: String::from("Shiv Sharma"),
        active: true,
        sign_in_count: 0,
    };

    let mut user2: User = User {
        email: String::from("shiv.sharma@zeeve.com"),
        username: String::from("Shiv Sharma"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1 email: {}", user1.email);
    println!("User1 name: {}", user1.username);
    println!("User1 active status: {}", user1.active);
    println!("User1 sign in count: {}", user1.sign_in_count);

    user2.email = String::from("newemail.com");
    user2.username.push_str("Bob");
    println!("User2 email: {}", user2.email);
    println!("User2 name: {}", user2.username);
    println!("User2 active status: {}", user2.active);
    println!("User2 sign in count: {}", user2.sign_in_count);

    // according to the ownership rules
    // let str = user2.email;
    // println!("User1 email: {}", user2.email); // now value borrowed and move out. give error
    let status = user2.active;
    println!("User2 active status: {}", user2.active);

    let str = user2.email;
    user2.email = String::from("New value");
    println!("User2 email: {}", user2.email);

    // whole struct should be mutable, we can not make one member of struct mutable

    let user3: User = build_user(
        String::from("axshivam"),
        String::from("shivatcoding@gmail.com"),
    );

    println!("User3 email: {}", user3.email);
    println!("User3 name: {}", user3.username);
    println!("User3 active status: {}", user3.active);
    println!("User3 sign in count: {}", user3.sign_in_count);

    let user4: User = User {
        email: String::from("user4@gmail.com"),
        ..user3
    };

    println!("User4 email: {}", user4.email);
    println!("User4 name: {}", user4.username);
    println!("User4 active status: {}", user4.active);
    println!("User4 sign in count: {}", user4.sign_in_count);

    // this will give error as ownership transferred
    // println!("User3 name: {}", user3.username);
    println!("User3 sign in count: {}", user3.sign_in_count);

    // Tuple struct
    let red: (u8, u8, u8) = (100, 0, 0);
    set_bg_color(red);
    let point: (u8, u8, u8) = (30, 40, 90);
    move_point(point);

    // now we can interchange the value that will provide doubts in long run
    let red: Color = Color(100, 0, 0);
    set_bg_color_updated(red);
    let point: Point = Point(30, 40, 90);
    move_point_updated(point);

    // calculate area of rectangle
    let w = 100;
    let h = 200;
    let area = calculate_area(w, h);

    println!("The area is {area}");

    // use tuple to solve issue in area
    let dimentions: (u32, u32) = (30, 40);

    println!("New using tuple area: {}", calculate_area_tuple(dimentions));

    let rectangle: Rectangle = Rectangle{
        width: 30,
        height: 40,
    };

    println!("New using tuple area: {}", calculate_area_struct(rectangle)); // now we are sending the ownership
    // so issue occur here, that we can not use rectangle now
    // println!("Width: {}", rectangle.width);

    let newRectangle: Rectangle = Rectangle{
        width: 30,
        height: 40,
    };

    println!("New using tuple area: {}", calculate_area_struct_with_borrow(&newRectangle));
    println!("Width: {}", newRectangle.width);

    println!("Rectangle: {:?}", newRectangle);
    println!("Rectangle: {:#?}", newRectangle);

    // dbg!(newRectangle); // but it takes the ownership of newRectangle
    dbg!(&newRectangle); // now it will not take ownership

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}

// RGB
fn set_bg_color(color: (u8, u8, u8)) {
    println!(
        "Setting Backgraoud Color R={}, G={}, B={}",
        color.0, color.1, color.2
    );
}

fn move_point(point: (u8, u8, u8)) {
    println!(
        "The cursor was moved x={}, y={}, z={}",
        point.0, point.1, point.2
    );
}

fn set_bg_color_updated(color: Color) {
    println!(
        "Setting Backgraoud Color R={}, G={}, B={}",
        color.0, color.1, color.2
    );
}

fn move_point_updated(point: Point) {
    println!(
        "The cursor was moved x={}, y={}, z={}",
        point.0, point.1, point.2
    );
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_tuple(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

fn calculate_area_struct(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn calculate_area_struct_with_borrow(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


