
#[cfg(test)]
mod test{
    use tokio::runtime;

    #[test]
    fn test(){
        let runtime1 = runtime::Builder::new_multi_thread().build().unwrap();
        for number in 1..100  {
            let future =  async move {
                println!("{}",number);
            };
            runtime1.spawn(future);
        }
        println!("{}","=================ok=================");
        runtime1.spawn(async  {
            println!("{}","ok,ok");
        });
    }
}