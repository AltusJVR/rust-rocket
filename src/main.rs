#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

mod world;

fn main() {
    rocket::ignite()
    .mount("/static", StaticFiles::from("static/index"))
    .mount("/", routes![index, world::world])
    .launch();
}