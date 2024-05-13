#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_host(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Cuboid {
    width: u32,
    height: u32,
    length: u32,
}

impl Cuboid {
    fn area(&self) -> u32 {
        2 * ((self.length * self.width) + (self.width * self.height) + (self.length * self.height))
    }
}

fn main() {
    let rectangle_1 = Rectangle {
        width: 20,
        height: 20,
    };

    let rectangle_2 = Rectangle {
        width: 10,
        height: 18,
    };

    let rectangle_3 = Rectangle {
        width: 25,
        height: 45,
    };

    let rectangle_4: Rectangle = Rectangle::square(10);

    let cuboid_1 = Cuboid {
        length: 20,
        width: 20,
        height: 20,
    };

    let cuboid_2 = Cuboid {
        length: 50,
        ..cuboid_1
    };

    // println!("Rectangle 1 is {:#?}", rectangle_1);

    // dbg!(&rectangle_1);

    println!("Rectangle 1 Area in square pixels = {}", rectangle_1.area());

    println!("Rectangle 2 Area in square pixels = {}", rectangle_2.area());

    println!("Rectangle 3 Area in square pixels = {}", rectangle_3.area());

    println!("Rectangle 4 Area in square pixels = {}", rectangle_4.area());

    println!("Rectangle 1 has non-zero width = {}", rectangle_1.width());

    println!(
        "Rectangle 1 can hold rectangle 2 = {}",
        rectangle_1.can_host(&rectangle_2)
    );

    println!(
        "Rectangle 1 can hold rectangle 3 = {}",
        rectangle_1.can_host(&rectangle_3)
    );

    println!("Cuboid 1 Area in square pixels = {}", cuboid_1.area());

    println!("Cuboid 2 Area in square pixels = {}", cuboid_2.area());
}
