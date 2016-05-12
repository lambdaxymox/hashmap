/// Helper macros that don't belong in any particular package.


/** 
This macro makes it easier to generating hash tables directly.

Example:
...
# #[macro_use] extern crate util;
# fn main() {
#     let table = hash_map!({ 1 => "One", 2 => "Two", 3 => "Three"});
#
#     println!("{} => {:?}", 1, table.get(&1));
#     println!("{} => {:?}", 2, table.get(&2));
#     println!("{} => {:?}", 3, table.get(&3));
# }
...
*/
#[macro_export]
macro_rules! hash_map(
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

);
