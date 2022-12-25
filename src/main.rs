extern crate core;

use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut height = String::new();
    let mut width = String::new();

    println!("Input width: ");
    io::stdin()
        .read_line(&mut width)
        .expect("Invalid input");

    println!("Input height: ");
    io::stdin()
        .read_line(&mut height)
        .expect("Invalid input");

    let width: u32 = match width.trim().parse() {
        Ok(x) => x,
        Err(err) => {
            println!("Error: {}", err);
            return
        }
    };

    let height: u32 = match height.trim().parse() {
        Ok(x) => x,
        Err(err) => {
            println!("Error: {}", err);
            return
        }
    };

    let rectangle: Rectangle = Rectangle {
        width,
        height
    };

    println!("Rectangle area is: {}", area(&rectangle));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
