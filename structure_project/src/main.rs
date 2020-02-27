mod service;
mod lib;
fn main() {
    println!("Hello, world!");
    service::user_service::create();

    lib::create_lib();
}
