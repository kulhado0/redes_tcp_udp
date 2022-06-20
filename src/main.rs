#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

mod domain;
mod consts;

use domain::commons;
use consts::game_tiles;

#[get("/")]
fn index() -> String {
    format!("{} {} {}", 
        game_tiles::BLOCKED.symbol().unwrap(), 
        game_tiles::POINT_OBJECT.symbol().unwrap(), 
        game_tiles::POWER_UP.symbol().unwrap())
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