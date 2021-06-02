#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/")]
fn say_hello() -> &'static str{
    "Hey, i am the server"
}

fn main() {
    println!("Starting server");
    rocket::ignite().mount("/", routes![say_hello]).launch();
}
