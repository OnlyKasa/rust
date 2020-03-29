#![allow(dead_code)]
use std::mem;

enum Number {
    Fix,
    Zero,
    One, 
    Two,
}

enum Color {
    Red = 0xf00,
    Green = 0x0f0,
    Blue = 0x00f,
}
fn get_color (color:Color) {
    match color {
        Color::Red => {
            println!("{}", "red");
        },
        Color::Blue => {
            println!("{}", "blue");
        },
        Color::Green => {
            println!("{}", "green");
        }
    }
}
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0, y: 0 })
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

fn main() {
    let too_long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', 2);
    println!("too long tuple: {:?}", too_long_tuple);
    println!("fix is {}", Number::Fix as i32);
    println!("zero is {}", Number::Zero as i32);
    get_color(Color::Red);
    get_color(Color::Blue);

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    let boxx: Box<Point> = *box_in_a_box;
    println!("Point occupies {} bytes on the stack",
             mem::size_of_val(&boxx));
    println!("{:?}",unbox(boxx));
}