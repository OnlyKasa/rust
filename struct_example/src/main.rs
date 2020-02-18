struct User {
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool
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

}


fn build_user (username: String, email: String) -> User{
    return User{
        username,
        email,
        sign_in_account: 1,
        active: true
    }
}