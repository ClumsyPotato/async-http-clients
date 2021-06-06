#![feature(proc_macro_hygiene, decl_macro)]

use core::time;
use std::thread::{self};

use std::io::Cursor;
use rand::Rng;
use rocket::response::content::Json;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};

#[macro_use] extern crate rocket;

static mut count :u16 = 0;

#[get("/")]
fn say_hello() -> &'static str{
    "Hey, i am the server"
}

#[get("/nonblocking")]
fn random_number() -> String{
  rand::random::<u8>().to_string()
}

#[get("/blocking")]
fn random_blocking() -> Json<Blocked>{
  let duration = rand::thread_rng().gen_range(0..10);
  thread::sleep(time::Duration::from_secs(duration));
  duration.to_string();
  unsafe {
    count = count +1;
    let r = Blocked{ 
      req_num: count,
      time_blocked: duration
    };
    Json(r)
  }
}

struct Blocked{
  req_num: u16,
  time_blocked: u64
}

impl<'r> Responder<'r> for Blocked {
  fn respond_to(self, _: &Request) -> response::Result<'r> {
    Response::build()
        .sized_body(Cursor::new(format!("{}:{}", self.req_num, self.time_blocked)))
        .ok()
  }
}

fn main() {
    println!("Starting server");
    rocket::ignite()
    .mount("/", routes![say_hello])
    .mount("/random", routes![random_number])
    .mount("/random", routes![random_blocking])
    .launch();
}
