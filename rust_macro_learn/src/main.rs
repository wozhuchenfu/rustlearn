use rust_proc_macro;
/*
1、过程宏必须定义在一个独立的crate中。不能在一个crate中既定义过程宏，又使用过程宏。

原理：过程宏是在编译一个crate之前，对crate的代码进行加工的一段程序，这段程序也是需要编译后执行的。如果定义过程宏和使用过程宏的代码写在一个crate中，那就陷入了死锁:

要编译的代码首先需要运行过程宏来展开，否则代码是不完整的，没法编译crate。
不能编译crate，crate中的过程宏代码就没法执行，就不能展开被过程宏装饰的代码
2、过程宏必须定义定义在lib目标中，不能定义在bin目标中

*/
/*#[derive(rust_proc_macro::Builder)]
struct Command {
    // ...
}*/

/*#[rust_proc_macro::sorted]
enum Letter {
    A,
    B,
    C,
    // ...
}*/

// 属性过程宏：空属性
#[rust_proc_macro::custom_proc_macro_attribute]
fn custom_proc_macro_attribute_fn() {
    println!("this is custom_proc_macro_attribute_fn()");
}

// 属性过程宏：自定义属性
#[rust_proc_macro::custom_proc_macro_attribute("this is custom_proc_macro_attribute")]
fn custom_proc_macro_attribute_fn_ha_attribute() {
    println!("this is custom_proc_macro_attribute_fn()");
}

#[derive(rust_proc_macro::CuctomProcMacroDerive)]
struct Student;

fn main() {
    rust_proc_macro::custom_proc_macro!();
    let a = rust_proc_macro::custom_proc_macro!(123);
    println!("{:?}",a);

    // 派生宏中派生的新方法，可以直接使用。
    let derive_fn = cuctom_proc_macro_derive_fn();
    println!("cuctom_proc_macro_derive_fn return:{}",derive_fn);

    custom_proc_macro_attribute_fn();
    custom_proc_macro_attribute_fn_ha_attribute();
    /*rust_proc_macro::seq! { n in 0..10 {
            /* ... */
        }}*/
    let macro21 = rust_proc_macro::macro2!("hello");
    println!("{}",macro21);
    println!("Hello, world!");
}
