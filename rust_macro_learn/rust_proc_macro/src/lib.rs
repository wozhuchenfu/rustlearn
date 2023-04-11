extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Item, DeriveInput};

#[proc_macro]
pub fn macro2(input:TokenStream)->TokenStream{
    println!("{:#?}",input);
    input
}

//过程宏必须定义在一个独立的crate中。不能在一个crate中既定义过程宏，又使用过程宏。
//
// 原理：过程宏是在编译一个crate之前，对crate的代码进行加工的一段程序，这段程序也是需要编译后执行的。如果定义过程宏和使用过程宏的代码写在一个crate中，那就陷入了死锁:
//
// 要编译的代码首先需要运行过程宏来展开，否则代码是不完整的，没法编译crate。
// 不能编译crate，crate中的过程宏代码就没法执行，就不能展开被过程宏装饰的代码
// 2、过程宏必须定义定义在lib目标中，不能定义在bin目标中

// 属性宏（Attribute macro）：用在结构体、字段、函数等地方，为其指定属性等功能。如标准库中的#[inline]、#[derive(...)]等都是属性宏。

#[proc_macro_attribute]
pub fn custom_proc_macro_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 使用parse_macro_input!宏将attr转换为语法树
    eprintln!("custom_proc_macro_attribute： {:#?}", parse_macro_input!(attr as AttributeArgs));
    // 转换成语法树
    let body_ast = parse_macro_input!(item as Item);
    // eprintln!("{:#?}", body_ast);

    match body_ast {
        // 获取到函数及相关属性
        Item::Fn(ref o) => {
            eprintln!("o:{:#?}", o);
            eprintln!("vis:{:#?}", o.vis); // 可见性
            eprintln!("attrs:{:#?}", o.attrs); // 特性
            eprintln!("sig:{:#?}", o.sig); // 方法签名
            eprintln!("block:{:#?}", o.block); // 方法体
            let x = o.block.clone();
            let iter = x.stmts.iter();
            for sm in iter.enumerate() {
                let x1 = sm.1;
            }

        }
        _ => {}
    }

    // quote!宏将语法树重新转换为TokenStream
    // 返回的实际结果为proc_macro2::TokenStream，要into转换为proc_macro::TokenStream
    quote!(#body_ast).into()

    // 返回空内容，将清楚函数内容
    // quote!({}).into()
}

// 属性式过程宏 custom_proc_macro_attribute 为宏的名字
#[proc_macro_attribute]
pub fn custom_proc_macro_attribute1(
    attrs: proc_macro::TokenStream, // 过程宏传的属性
    input: proc_macro::TokenStream, // 作用的代码
) -> proc_macro::TokenStream {
    eprintln!("attrs:{:#?}", attrs); // 注意输出要用epringln!
    eprintln!("input:{:#?}", input);
    input

    // 如果返回空对象，相当于去除方法
    // proc_macro::TokenStream::new()
}

// 派生式过程宏 CuctomProcMacroDerive 为宏名称
#[proc_macro_derive(CuctomProcMacroDerive)]
pub fn cuctom_proc_macro_derive(input: TokenStream) -> TokenStream {
    eprint!("cuctom_proc_macro_derive:{:#?}", input);

    // 派生一个叫cuctom_proc_macro_derive_fn的方法
    "fn cuctom_proc_macro_derive_fn() -> i32 {100}"
        .parse()
        .unwrap()

    // 返回空代表没派生内容
    // proc_macro::TokenStream::new()
}

// 修改定义中的代码，使用syn和quote库来将TokenStream转化为语法树，再对语法树进行处理。
/*#[proc_macro_derive(CuctomProcMacroDerive)]
pub fn cuctom_proc_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 转换成语法树，注意这里的类型为DeriveInput
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident; // 名称
    let attrs = input.attrs; // 特性
    let vis = input.vis; // 可见性
    let generics = input.generics; // 泛型
    let data = input.data; // 数据

    // 生成输出
    let expanded = quote! {
      fn cuctom_proc_macro_derive_fn() -> i32 {100}

      impl Trait for #name {
        fn print(&self) -> usize {
            println!("{}","hello from #name")
        }
      }
    };

    // 返回TokenStream
    expanded.into()
    // 等同于
    // proc_macro::TokenStream::from(expanded)
}*/

// 函数式过程宏 custom_proc_macro 为宏名称
#[proc_macro]
pub fn custom_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // eprint!("custom_proc_macro:{:#?}", input);
    // "fn custom_proc_macro_fn() -> i32 {100}".parse().unwrap()

    // 使用syn和quote库来将TokenStream转化为语法树，再对语法树进行处理
    quote!({
        fn custom_proc_macro_fn() -> i32 {
            100
        }
    }).into()
    // proc_macro::TokenStream::new()
}

macro_rules! my_print {
        ($($a:expr),*) => {
            $(
                println!("{}",$a);
            )*
        };
    }
/*
常见的 Rust宏选择器：
item：条目，例如函数、结构、模块等
block：代码块
stmt：语句
pat：模式
expr：表达式
ty：类型
ident：标识符
path：路径，例如 foo、 ::std::mem::replace, transmute::<_, int>, …
meta：元信息条目，例如 #[…]和 #![rust macro…] 属性
tt：词条树

*/
macro_rules! min {
    ($a:expr) => {$a};
    ($a:expr,$($b:tt)*)=>{
        std::cmp::min($a,min!($($b)*))
    };
}

macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name(){
            println!("{:?}()",stringify!($func_name));
        }
    )
}
create_function!(foo);


// 过程宏分为三种：
//
// 派生宏（Derive macro）：用于结构体（struct）、枚举（enum）、联合（union）类型，可为其实现函数或特征（Trait）。
// 属性宏（Attribute macro）：用在结构体、字段、函数等地方，为其指定属性等功能。如标准库中的#[inline]、#[derive(...)]等都是属性宏。
// 函数式宏（Function-like macro）：用法与普通的规则宏类似，但功能更加强大，可实现任意语法树层面的转换功能。

// 派生宏
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}


// 属性宏
#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}


// 函数宏
#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}

#[cfg(test)]
mod marco1 {
    use super::*;

    #[test]
    fn test(){

        foo();
        // my_print!(1,2,3,4);
        let arr = vec![1,2,3,4,5];
        let min = min!(1,2,3,4,5);
        println!("{:?}",min);
        println!("{}","rust macro_rules");
    }
}