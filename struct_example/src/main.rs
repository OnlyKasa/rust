#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // add code here
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    let mut user = User {
        username: String::from("somone"),
        email: String::from("someone@example.com"),
        sign_in_account: 1,
        active: true
    };

    println!("{}", user.active);
    user.active = false;
    println!("{}", user.active);
    println!("{}", user.username);

    let user = build_user(String::from("usernametring"), 
    String::from("emailtring"));
    println!("{}", user.username);

    let user2 = User {
        ..user
    };
    println!("{}", user2.username);

    // function with 2 variable
    println!("the area = {}", areaOrigin(30, 50));


    // refactoring variable by tuple 
    let rect1 = (30, 50);
    println!("the area = {}", area(rect1));

    // refactoring variable by struct 
    let rect2 = Rectangle {
        width: 30, 
        height: 50
    };
    println!("the area = {}", areaStruct(rect2));


    let rect3 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );
}

fn areaStruct(dimensions: Rectangle) -> u32{
    dimensions.width * dimensions.height
}

fn areaOrigin(width: u32, height: u32) ->u32 {
    width * height
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn build_user (username: String, email: String) -> User{
    return User{
        username,
        email,
        sign_in_account: 1,
        active: true
    }
}