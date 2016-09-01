/// This macro generates HashMaps directly.
///
/// # Examples
///
///```rust
/// #[macro_use] extern crate hashmap;
///
/// fn main() {
///     let table = hashmap!(1 => "One", 2 => "Two", 3 => "Three");
///
///      println!("{} => {:?}", 1, table.get(&1));
///      println!("{} => {:?}", 2, table.get(&2));
///      println!("{} => {:?}", 3, table.get(&3));
/// }
///```
#[macro_export]
macro_rules! hashmap(
    { } => {
        {
            use std::collections::HashMap;

            HashMap::new()
        }
    };

    { $ ( $key : expr => $value : expr ) , + } => {
        {
            use std::collections::HashMap;

            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };

    { $ ( $key : expr => $value : expr , ) + } => {
        {
            use std::collections::HashMap;

            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };

    ( )  => {
        {
            use std::collections::HashMap;

            HashMap::new()
        }
    };

    ({ })  => {
        {
            use std::collections::HashMap;

            HashMap::new()
        }
    };

    ({ $ ( $key : expr => $value : expr ) , + }) => {
        {
            use std::collections::HashMap;

            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };

    ({ $ ( $key : expr => $value : expr , ) + }) => {
        {
            use std::collections::HashMap;

            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };

);
