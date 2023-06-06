#![feature(decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::http::Cookies;
use serde::Deserialize;
use rocket_contrib::json::Json;


#[derive(Deserialize)]
struct SomeJSON{
    name: String,
    age: u8,
}

#[post("/json", data = "<json>")]
fn pass_json(json: Json<SomeJSON>) -> String{
    format!("Name: {}, Age: {}", json.name, json.age)
}

#[get("/resource/<id>")]
fn set_cookie(id:String, mut cookies: Cookies) -> String{
    cookies.add_private(rocket::http::Cookie::new("user_id", id));
    "Cookie set".to_string()
}

#[get("/resource")]
fn get_cookie(mut cookies: Cookies) -> String{
    let user_id = cookies.get_private("user_id").unwrap();
    format!("User id is {}", user_id.value())
}
use rocket::request::{Form, FromForm};

#[derive(FromForm)]
struct UserData{
    name: String,
    age: String,
    email: String,
    password: String,
}

use rocket::response::NamedFile;

#[get("/form")]
fn form() -> NamedFile {
    NamedFile::open("static/index.html").unwrap()
}


#[post("/doSomething", data = "<param>")]
fn do_something(param: Form<UserData>) -> String {
    format!("Name: {}, Age: {}, Email: {}, Password: {}", param.name, param.age, param.email, param.password)
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}


#[get("/auth?<username>&<password>")]
fn authenticate(username: &RawStr, password: &RawStr) -> String{
    format!("Username: {}, Password: {}", username.as_str(), password.as_str())
}

use rocket::response::status;
#[post("/<id>")]
fn few (id: usize) -> status::Accepted<String>{
    status::Accepted(Some(format!("Hello {}", id)))
}

fn main(){
    let server = rocket::ignite();
    let server = server.mount("/", routes![index, set_cookie, get_cookie, form, do_something, pass_json, few,authenticate]);
    let server = server.launch();
}