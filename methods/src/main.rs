#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perim(&self) -> u32 {
        return 2 * (self.width + self.height);
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return (self.width > other.width) && (self.height > other.height);
    }

    fn square(size: u32) -> Self {
        Rectangle {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    println!(
        "R1: Area {} Perimeter {}",
        rect1.area(),
        rect1.perim()
    );

    println!("R1 can hold R2 : {}", rect1.can_hold(&rect2));
    println!("R2 can hold R1 : {}", rect2.can_hold(&rect1));

    let square = Rectangle::square(5);

    println!("Square area: {}", square.area());
}