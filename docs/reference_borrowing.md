# Reference 
``` rust
 let foo = String;
 // define references 
 let foo_refer = &foo;
 // define references [mut]
 let foo_mut_refer = &mut foo;
```

# Mutable reference
# Slice
Khi sử dụng slice thì việc vay mượn kết thúc tức là giá trị biến ban đầu không còn tồn tại nữa.
``` rust
    let mut s = String::from("hello world");
    let word = s[..2]; // slice func
    s.clear(); // error!
```