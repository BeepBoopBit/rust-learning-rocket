#![feature(decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;

#[get("/")]
fn my_first_route() -> String{
    String::from("Hello, Rocket!")
}

#[get("/earth")]
fn the_earth() -> String{
    String::from("You've landed in earth")
}

// <snippet-code>
#[get("/earth/<continent>")]
fn earth_continent(continent: &RawStr) -> String{
    format!("You've landed in earth at {} continent", continent)
}

use std::path::PathBuf;
#[get("/page/<path..>")]
fn get_page(path: PathBuf) -> String {
    format!("You've landed in page: {}", path.display())
}

use rocket::response::NamedFile;
use std::path::Path;
#[get("/secured/<path..>")]
fn get_secured_page(path: PathBuf) -> Option<NamedFile>{
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn main(){
    let server = rocket::ignite();
    let server = server.mount("/", routes![my_first_route, the_earth, earth_continent, get_page, get_secured_page]);
    let server = server.launch();
}