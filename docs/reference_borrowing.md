# Reference 
``` rust
 let foo = String;
 // define references 
 let foo_refer = &foo;
 // define references [mut]
 let foo_mut_refer = &mut foo;
```

# Mutable reference
<b>Note: </b> 
1. Không sử dụng liên tiếp khai báo reference và mutable reference hoặc nhiều mutable reference

``` rust
    let s = String::from("This is example");
    // example 1
    let r1 = &mut s;
    let r2 = &mut s; // error

    // example 2
    let r3 = &s;
    let r4 = &mut s; // error
```
2.  Khi reference tới 1 vùng bộ nhớ thì vùng bộ nhớ đó phải chưa giải phóng.

``` rust
 fn  dangle () -> &String{
     let s = String::from("hello");
     return &s  // error
 } // s đã được giải phóng nhưng hàm dangle() đang trả về 1 reference tới s
```
# Slice (another reference)

``` rust
    let mut s = String::from("hello world");
    let word = s[..2] // word will get the value 'world'
    s.clear(); // this empties the String, making it equal to ""

    // s đã được giải phóng sau khi clear nên 'word' là một kiểu reference nên cũng bị giải phóng theo.
    println!(word) // error
```