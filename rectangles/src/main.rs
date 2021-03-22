// deriving the Debug trait for being able to use !println macro on struct
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// #############
// Method Syntax
// #############

// methods can take ownership of self (rare) or even just borrow
// just like with any other parameter

// implementation block
impl Rectangle {
    fn area(&self) -> u32 { //&self used because Rust knows that the type is Reactangle anyway
        self.width * self.height
    }

    // associated functions 
    // --------------------
    // can define functions that don't take self as a parameter 
    // BUT still associated with the struct
    // used for constructors 
    fn square(size: u32) -> Rectangle {
        // returns a new instance of the Rectangle object as a square
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Also valid to have multiple implementation blocks for same struct

impl Rectangle {
    // Method with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // use of an associative method
    let _sq = Rectangle::square(3);

    // by default, structs have no clear way of being printed
    // Rust doesnt try to guess what we want, no implementation of struts for Display formatting
    println!("rect1 is {:#?}", rect1);

    // {:?} - Debug print formatting BUT need to explicitly state this will be used --> #[derive(Debug)]
    // {:#?} a more prettier debug printing format

    let rect_area = area(rect1.width, rect1.height);

    println!("The area is {}", rect_area);

    // using the implementation made for Reactangle struct (method syntax)
    println!("The area is {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
