use rust_proc_macro;
/*
1、过程宏必须定义在一个独立的crate中。不能在一个crate中既定义过程宏，又使用过程宏。

原理：过程宏是在编译一个crate之前，对crate的代码进行加工的一段程序，这段程序也是需要编译后执行的。如果定义过程宏和使用过程宏的代码写在一个crate中，那就陷入了死锁:

要编译的代码首先需要运行过程宏来展开，否则代码是不完整的，没法编译crate。
不能编译crate，crate中的过程宏代码就没法执行，就不能展开被过程宏装饰的代码
2、过程宏必须定义定义在lib目标中，不能定义在bin目标中

*/

fn main() {
    let macro21 = rust_proc_macro::macro2!("hello");
    println!("{}",macro21);
    println!("Hello, world!");
}
