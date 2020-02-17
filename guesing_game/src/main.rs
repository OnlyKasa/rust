use std::io;
fn main() {

    let char = '😂';

    println! ("{}", char);

    let s = call_me_for_test(); 
    println!("{}", s);

    let mut s = String::from("hello world");

    let word = first_word(&mut s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}


fn call_me_for_test() -> usize{

    let mut s = String::from("hello");
    let x = 5 ;
    let y = x;
    println!("{}", y);

    let s1 = String::from("this is s1");
    let mut s2 = s1.clone();
    s2.push_str(" + s2 plus");
    println!("s1 = {},  s2 = {}",s1 ,s2);
    s.push_str(" world!!");

    let spaces = "   ";
    let spaces = spaces.len();

    spaces
}


fn first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}
