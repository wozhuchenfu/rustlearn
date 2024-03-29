extern crate proc_macro;
extern crate regex;
extern crate md5;

use crate::proc_macro::TokenStream;

use quote::quote;
use syn;
use std::str::FromStr;
use md5::{Md5, Digest};


///
///
/// NotEmpty : not null or empty
/// Length(1,3) : String's length between (min, max)
/// Min(18) : integer min value
/// Max(20) : integer max value
/// Range(2,5) : integer value between (min, max)
/// Email : is email address or not
/// Pattern(r"^.*$") : regular expression
///
#[proc_macro_derive(BeanCheck1, attributes(NotEmpty, Length, Min, Max, Range, Email, Pattern))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_validate(&ast)
}

fn generate_validate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut validate_quote = quote! {};

    let mut fv_map_quote = quote! {
        let mut mm = BTreeMap::new();
    };

    match &ast.data {
        syn::Data::Struct(ds) => {
            match &ds.fields {
                syn::Fields::Named(ff) => {

                    ff.named.iter().for_each(|f| {

                        let ident = &f.ident;

                        fv_map_quote = quote! {
                            #fv_map_quote
                            mm.insert(stringify!(#ident), format!("{}", self.#ident));
                        };

                        let ty = &f.ty;
                        let attrs = &f.attrs;
                        let mut ft = String::from("");
                        match ty {
                            syn::Type::Path(p) => {
                                ft = format!("{}", p.path.segments[0].ident);
                            },
                            _ => {},
                        }

                        attrs.iter().for_each(|at| {
                            let mut prop = String::from("");
                            let mut param = String::from("");

                            if !at.path.segments.is_empty() && at.path.segments.len() > 0 {
                                prop = format!("{}", at.path.segments[0].ident);
                            }

                            if !at.tokens.is_empty() {
                                param = format!("{}", at.tokens);
                            }

                            let tmp = param.replace("(", "").replace(")", "").replace(" ", "");

                            // handle String field
                            if ft == "String".to_owned() {
                                // handle property : Length(1,2)
                                if prop == "Length".to_string() && param.len() > 0 {
                                    let mut min = 0usize;
                                    let mut max = 0usize;

                                    let rst: Vec<&str> = tmp.split(",").collect();
                                    if rst.len() == 2 {
                                        min = u32::from_str(rst[0]).unwrap_or(0) as usize;
                                        max = u32::from_str(rst[1]).unwrap_or(0) as usize;
                                    }
                                    validate_quote = quote! {
                                        #validate_quote
                                        if !( self.#ident.len() >= #min && self.#ident.len() <= #max ) {
                                            return Err(bean_check_lib::CheckError::Simple(stringify!(check failed: #ident #at).to_string()));
                                        }
                                    }
                                } else if prop == "Pattern".to_string() && param.len() > 0 {
                                    // handle property : Pattern(r"^.*$")
                                    validate_quote = quote! {
                                        #validate_quote
                                        let g = #param;
                                        let reg_str = &g[3..(g.len()-2)];
                                        if !( Regex::from_str(reg_str).unwrap().is_match(self.#ident.as_str())  ) {
                                            return Err(bean_check_lib::CheckError::Simple(stringify!(check failed: #ident #at).to_string()));
                                        }
                                    }
                                }  else if prop == "Email".to_string() {
                                    println!("check mail");
                                    validate_quote = quote! {
                                        #validate_quote
                                        if !( Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap().is_match(self.#ident.as_str())  ) {
                                            return Err(bean_check_lib::CheckError::Simple(stringify!(check failed: #ident #at).to_string()));
                                        }
                                    }
                                }

                            } else if ft == "u8".to_string() || ft == "i8".to_string() || ft == "u16".to_string() || ft == "i16".to_string() || ft == "u32".to_string() || ft == "i32".to_string() || ft == "u64".to_string() || ft == "i64".to_string() || ft == "u128".to_string() || ft == "i128".to_string() || ft == "f32".to_string() || ft == "f64".to_string() {
                                // handle Integer field
                                if prop == "Min".to_string() {
                                    let min = i32::from_str(tmp.as_str()).unwrap_or(0);
                                    validate_quote = quote! {
                                        #validate_quote
                                        if !( self.#ident >= #min  as #ty ) {
                                            return Err(bean_check_lib::CheckError::Simple(stringify!(check failed: #ident #at).to_string()));
                                        }
                                    }
                                } else if prop == "Max".to_string() {
                                    let max = i32::from_str(tmp.as_str()).unwrap_or(0);
                                    validate_quote = quote! {
                                        #validate_quote
                                        if !( self.#ident <= #max  as #ty ) {
                                            return Err(bean_check_lib::CheckError::Simple(stringify!(check failed: #ident #at).to_string()));
                                        }
                                    }
                                } else if prop == "Range".to_string() {
                                    let rst: Vec<&str> = tmp.split(",").collect();
                                    if rst.len() == 2 {
                                        let min = i32::from_str(rst[0]).unwrap_or(0);
                                        let max = i32::from_str(rst[1]).unwrap_or(0);
                                        validate_quote = quote! {
                                            #validate_quote
                                            if !( self.#ident >= (#min as #ty) && self.#ident <= (#max as #ty) ) {
                                                return Err(bean_check_lib::CheckError::Simple(stringify!(check failed: #ident #at).to_string()));
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }


    TokenStream::from(
        quote! {
            impl bean_check_lib::BeanCheck for #name {
                fn validate(&self) -> std::result::Result<(), CheckError> {
                    #validate_quote
                    Ok(())
                }

                fn sign_check(&self, uri:&String, token:&String)->bool {
                    #fv_map_quote
                    let mut vv = vec![];
                    let mut client_sign = "".to_string();

                    for (k, v) in mm {
                        if k == "sign".to_string() {
                            client_sign = v;
                        }
                        else {
                            vv.push(v);
                        }
                    }
                    let values = vv.concat();

                    let salt = "45eb49de-e591-4f77-9172-044d0fa10d83";

                    let plain = format!("{}{}{}{}", uri, &values, token, &salt);
                    println!("plain : {}", &plain);
                    return true;
                    /*let mut hasher = md5::Md5::new();
                    hasher.input(plain.as_bytes());
                    let server_sign = format!("{:x}", hasher.result());

                    println!("server sign : {},  client sign :{}", &server_sign, &client_sign);

                    client_sign == server_sign*/
                }
           }
        }
    )
}