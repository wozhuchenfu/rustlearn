use std::collections::HashMap;


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


macro_rules! map {
    ($($key:expr => $value:expr),*) => (
        {
            let mut hm = std::collections::HashMap::new();
            $(hm.insert($key,$value);)*;
            hm
        }
    );
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("{:?}()",stringify!($func_name));
        }
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn test(){
        create_function!(name);
        name();
        let map1 = map!("a"=>"a1","b"=>"b1");
        println!("{:?}",map1);
        let mut ve = my_vec!["hi"];
        ve.push("hello");
        println!("{:?}",ve);
    }
}