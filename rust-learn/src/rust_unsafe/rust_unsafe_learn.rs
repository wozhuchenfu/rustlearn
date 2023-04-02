
#[cfg(test)]
mod test{

    static HELLO_WORLD:&str = "hello world";
    static mut COUNTER:i32 = 0;
    fn add_counter(v:i32){
        unsafe{
            COUNTER+=v;
        }
    }

    fn return_closure(b:i32)-> Box<dyn Fn(i32)->i32>{
        Box::new(move |a| a+b)
    }
    //可变裸指针 *mut T 不可变裸指针 *const T
    #[test]
    fn test(){
        let mut num = 5;
        let r1 = &num as *const i32;
        let mut r2 = &mut num as *mut i32;

        add_counter(9);
        let closure = return_closure(3);
        println!("{}",closure(2));
        unsafe{
        println!("{}",COUNTER);
            println!("r1 is:{}",*r1);
            *r2 = 9;
            println!("r2 is:{}",*r2);
            dangerous();
        }
        // println!("num:{},r1:{},r2:{}",num,*r1,*r2);
    }
    unsafe fn dangerous(){
        println!("{}","do something dangerous");
    }
}