#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

mod domain;
mod consts;

use domain::commons;
use consts::tiles;

#[get("/")]
fn index() -> String {
    format!("hello")
}

#[get("/?<x>&<y>")]
fn tile_symbol(x: usize, y: usize) -> String {
    format!("({x}, {y})")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/tile-symbol", routes![tile_symbol])
        .launch();
}