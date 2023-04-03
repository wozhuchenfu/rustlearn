#[macro_export]
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}



#[cfg(test)]
mod test{
    #[test]
    fn test(){
        let mut ve = my_vec!["hi"];
        ve.push("hello");
        println!("{:?}",ve);
    }
}