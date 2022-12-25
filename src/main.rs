extern crate core;

use core::panicking::panic;
use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut height = String::new();
    let mut width = String::new();

    io::stdin()
        .read_line(&mut width)
        .expect("Invalid input");

    io::stdin()
        .read_line(&mut height)
        .expect("Invalid input");

    let width: u32 = match width.parse() {
        Ok(x) => x,
        Err(err) => panic!("Invalid height: {}", err)
    };

    let height: u32 = match height.parse() {
        Ok(x) => x,
        Err(err) => panic!("Invalid height: {}", err)
    };
}
