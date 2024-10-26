#[derive(Debug)]
#[warn(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        // we can take extra arguments, without & it takes ownership
        self.height * self.width
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return (self.width * self.height) > rect.width * rect.height;
    }

    fn square(side: u32) -> Self { // Self == Rectangle
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };

    // let area = calculate_area(&rect);
    // println!("Area: {}", area);

    println!("Area: {}", rect.calculate_area());

    let rect1: Rectangle = Rectangle {
        width: 300,
        height: 40,
    };

    println!("Area: {}", rect1.calculate_area());
    println!("Method with same name as struct field: {}", rect1.width());

    let rect2: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };

    println!("rect1 can hold the rect2: {}", rect1.can_hold(&rect2));

    let sq: Rectangle = Rectangle::square(6);
    println!("{:?}", sq);
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
