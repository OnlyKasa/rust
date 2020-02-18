## Struct example

```rust
    // Khai báo
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    // khởi tạo
    // 1.  init by object
    let mut user = User {
        username: String::from("somone"),
        email: String::from("someone@example.com"),
        sign_in_account: 1,
        active: true
    };
    //2. init by function  username same user.username.
    fn build_user (username: String, email: String){
        return User{
            username,
            email,
            sign_in_account: 1,
            active: true
        }
    }
    // copy instance .. copy value to new object 
    let user2 = User {
        username: String::from("override user name");
        ..user
    }

```