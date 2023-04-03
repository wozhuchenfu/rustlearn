extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn macro2(input:TokenStream)->TokenStream{
    println!("{:#?}",input);
    input
}

#[cfg(test)]
mod marco1 {
    use super::*;
    #[macro_export]
    macro_rules! my_print {
        ($($a:expr),*) => {
            $(
                println!("{}",$a);
            )*
        };
    }

    #[test]
    fn test(){
        my_print!(1,2,3,4);
        println!("{}","rust macro_rules");
    }
}