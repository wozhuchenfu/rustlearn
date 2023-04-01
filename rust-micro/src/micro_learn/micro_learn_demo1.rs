// use proc_macro::TokenStream;
//
// #[proc_micro_derive(Builder)]
// pub fn derive_builder(input: TokenStream) -> TokenStream{
//
// }

pub mod test{
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::ops::{DerefMut, Deref};

    pub fn test1(a:&str) ->&str{
        println!("{}",a);
        a
    }

    pub struct human<'a>{
        name:&'a str,
        age:i32
    }

    #[derive(Debug)]
    pub enum list{
        cons(i32,RefCell<Box<list>>),
        null,
    }
    impl list{
        pub fn new()->list{
            list::cons(0,RefCell::new(Box::new(list::null)))
        }

        pub fn add(&mut self,val:i32){
            let mut temp = self;
            loop {
                match temp {
                    list::cons(a,b) => {
                        let x = b.get_mut();
                        temp = x
                    },
                    list::null => break,
                }
            }
            *temp = list::cons(val,RefCell::new(Box::new(list::null)));
        }
    }
}

#[cfg(test)]
mod tests{

    use super::test;
    use std::collections::LinkedList;

    #[test]
    fn test(){
        let mut linked_list = LinkedList::new();
        linked_list.push_back("a");
        let test1 = test::test1("hello");
        let mut list1 = test::list::new();
        list1.add(2);
        println!("{:?}",list1);
    }
    #[test]
    fn test2(){
        let mut x = Box::new(3);
        let mut z = &x;
        /*for i in 1..5*/ {
            x = Box::new(0);
            z = &x;
        }
        println!("{}",z);
    }

    struct StrSplit<'s,'p>{
        delimiter:&'p str,
        document:&'s str,
    }

    impl <'s,'p> Iterator for StrSplit<'s,'p>{
        type Item = &'s str;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    fn str_before(s:&str,c:char)->Option<&str>{
        StrSplit{
            document:s,
            delimiter:&c.to_string(),
        }.next()
    }

    struct Mustr<'a,'b>{
        s:&'a mut &'b str,
    }

    #[test]
    fn test3(){
        //&'static 可以替换 &'a
        // 函数参数要求越少可作用的范围越大 如果参数引用的生命周期是&'static 的那么只能传递&'static生命周期的引用
        // 如果参数引用的生命周期是&'a 的那么可以用&'static的生命周期的引用的参数传递给函数
        let mut r = "hello";// &'static r的生命周期
        *Mustr{s:&mut r}.s = "world";//&mut T 生命周期必须是指定类型
        println!("{}",r);
    }



    fn asfn1(){
        println!("{}","asfn1");
    }
}