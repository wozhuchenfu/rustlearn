mod mylib;

use std::collections::HashMap;
use mylib;
fn main() {
    mylib::my_mod::hello();
    let map:HashMap<&str,&str> = HashMap::new();
    println!("Hello, world!");
}
