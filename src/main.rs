use std::{io};



fn main() {
    println!("Hello, world!");
    let mut guss = String::new();
    io::stdin().read_line(&mut guss).expect("读取失败");
    print!("{}",guss)
}
