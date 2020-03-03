fn main() {
    println!("Hello, world!");
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1,2,3];

    println!("vector one {:#?}", v1);
    println!("vector two {:#?}", v2);


    // update vector
    let mut v = Vec::new();
    v.push("1");
    v.push("2");
   // v.push("3"); /// error. the value no same type with others value 

    let first = &v[0];

    // println!("first {:?}", first);

    v.push("3"); // error when update vector when holding a reference to an item
    println!("first {:?}", first);
}
