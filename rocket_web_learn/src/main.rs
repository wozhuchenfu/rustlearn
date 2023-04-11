#[macro_use] extern crate rocket;
use rocket::{Config,catch,catcher,delete,get,post,put,routes,serde::json::{Value,serde_json::json}};
use std::net::Ipv4Addr;


#[get("/")]
fn index()->&'static str{
    "hello, world"
}

#[get("/product")]
async fn get_product()->Value{
    json!("list")
}

#[get("/<id>")]
async fn get_product_by_id(id:i32)->Value{
    json!("list")
}

#[post("/")]
async fn create_product()->Value{
    json!("list")
}

#[put("/<id>")]
async fn put_product(id:i32)->Value{
    json!("list")
}

#[delete("/<id>")]
async fn delete_product(id:i32)->Value{
    json!("list")
}
#[catch(404)]
async fn not_found()->Value{
    json!("Not found!")
}

#[rocket::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{

    let config = Config{
        port: 8090,
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Default::default()
    };

    rocket::custom(config).mount("/",routes![index])
        .mount("/product",routes![get_product,
        get_product_by_id,create_product,
        put_product,delete_product])
        .register("/",catchers!(not_found))
        .launch().await?;
    Ok(())
}
