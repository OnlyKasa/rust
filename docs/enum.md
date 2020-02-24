# Define Enum
```rust
    enum EnumName {
        Value1,
        Value2,
        Value3
    }
    
```

# Option enum
``` rust
    enum Option<T> {
        Some(T),
        None
    }

    let op: Option<u32> = Some(4);
    let op_none: Option<u32> = None;

    match op {
        Some(value) => {
            println!("some value = {}", value);
        }
        None => {
            println!("this is none")
        }
    }
```