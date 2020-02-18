fn main() {
    // let foo =  String::from("a");
    // call_function(&foo);
    // println!("{}", call_function(&foo));

    // let mut s = String::from("hello");
    // let r2 = &mut s;
    // println!("{}", r2);
    // let r1 = &mut s;
    // r1.push_str("string: &str");
    // println!("{}", r1.len());
    // println!("{}", r1);

    slice();
}

fn call_function(_si : &String) -> usize {
    _si.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn slice() {
    let mut s = String::from("hello world");
    let _word = &s[..2]; // slice func
    s.clear(); // error!
    println!("{}", _word);
}
