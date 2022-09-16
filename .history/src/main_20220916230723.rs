mod mylib;

use std::collections::HashMap;
fn main() {
    mylib::my_mod::::hello();
    let map:HashMap<&str,&str> = HashMap::new();
    println!("Hello, world!");
}
