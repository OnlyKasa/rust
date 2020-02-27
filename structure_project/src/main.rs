use std::mem;
mod service;
mod lib;
fn main() {
    println!("Hello, world!");
    service::user_service::create();

    lib::create_lib();

    println!("1 - 2 = {}", 1 - 2);

    let long_tuple = (1u8, 2u16, 3u32,  4u64,
    -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64,'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    println!("tuple of tuples: {:?}", long_tuple);

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

