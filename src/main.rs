#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod recipe_manager;
mod web_api;

mod tests;

fn main() {
    println!("Hello, world!");
}
