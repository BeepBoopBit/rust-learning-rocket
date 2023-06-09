# Introduction

We'll first introduce ourselves with the basics of rocket.

## Adding Rocket in your cargo project

Type `cargo add rocket` in your terminal to add it in your project.

## Basics


### Starting a Rocket Server
To start a rocket server, we need to ignite it as follows:

```rust
// For now you don't need to understand these macros. Just put this in your project.
#[feature(decle_macro)]
#[macro_use] extern crate rocket;

fn main(){
    let server = rocket::ignite();
}
```

Igniting just starts the engine of our rocket, therefore, this would not start our venture. In order for us to launch it we need to first specify where would our rocket go by mounting it our system.

```rust
// <snippet-code>

#[get("/")]
fn my_first_route() -> String{
    String::from("Hello, Rocket!")
}

fn main(){
    let server = rocket::ignite();
    let server = server.mount("/", routes![my_first_route]);
    let server = server.launch();
}
```

Now when you run this program, you can now check `localhost:8000` to see `Hello, Rocket!` in your browser. But how does the code above work?

Above the `my_firs_route()` function, you can see a macro that specified its location. It's kind of like the coordinate of a planet for our rocket to venture to. This function returns a string to print in the browser.

`server.mount()` mounts the default route as well as all possible routes in the server. It's supposedly first go to the `/` as the default first venture otherwise, if we specified a particular path for our rocket, it would first go there.

`server.launch()` just lunches our server.`

### Having multiple routes

To have multiple routes that our rocket can take, we can just do the follow code

```rust
// <snippet-code>

#[get("/")]
fn my_first_route() -> String{
    String::from("Hello, Rocket!")
}

#[get("/earth")]
fn the_earth() -> String{
    String::from("You've landed in earth")
}

fn main(){
    let server = rocket::ignite();
    let server = server.mount("/", routes![my_first_route, the_earth]);
    let server = server.launch();
}

```

Now try to go to `http://localhost:8000/earth` and you'll see that you now landed in earth.

As said previously, `.mount` also mountes, different possible paths that our rocket can go. Try to change the default route from `/` to `/earth` and try to just go to `http://localhost:8000`

You didn't expect what happen didn't you? you now need to `/earth` to access the `my_first_route` and `/earth/earth` to access `the_earth`. That's because the default route is really called `base` where it's the basis of all the possible location you want to go.

### Dynamic Paths

Now let's explore how to create dynamic paths

```rust
use rocket::http::RawStr;

// <snippet-code>
#[get("/earth/<continent>")]
fn earth_continent(continent: &RawStr) -> String{
    format!("You've landed in earth at {} continent", continent)
}

fn main(){
    let server = rocket::ignite();
    let server = server.mount("/", routes![my_first_route, the_earth, earth_continent]);
    let server = server.launch();
}
```

Now, if you put `http://localhost:8000/earth/asia` you'll now see on your browser: `You've landed in earth at asia continent`.

By having `<variable-name>` you can then put the same name in the parameter list to accept information from the browser. It can be String (with allocation but decoded), RawStr (undecoded, no allocation), u32, f32, or any type really.

### Multiple Segments

We can also get multiple segments of the URL using `<param..>` in the route path:

```rust
use std::path::PathBuf;

#[get("/page/<path..>")]
fn get_last_path(path: PathBuf) -> String{
    format("You've landed at path: {}", path.dispay())
}
```

Now if we add this to our rocket routes and input `http://localhost:8000/page/static/index.html` you would see in your browser `You've landed at path: static/index.html`

However, this format can lead to path traversal attacks. Meaning they can make use of `../` to traverse into resources that is not suppose to be accessible. To mitiate with it, we can use `rocket::response::NamedFile`.

```rust

#[get("/secured/<file..>")]
fn get_secured_file(file: PathBuf) -> Option<NamedFile>{
    NamedFile::open(Path::new("Static/").join(file)).ok();
}
```

### Forwarding

Let's say you have a function as follows:

```rust
#[get("/<A>/<B>/<C>")]
fn something(A: bool, B: u32, C: String){
    // doing something
}
```

What will happen if `A` is not bool? or B is not `u32`?

When a parameter mismatch occurs, rocket forwards the request to another matching route. Just like an instance when there's multiple landing zone in one planet, your rocket needs to look for the right landing zone where it the right people can get your package.

```rust
#[get("/user/<id>")]
fn user(id: usize){}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize){}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &RawStr){}

```

rank signifies the precedence of checking the function.

### Query String

You can also specify query strings by doing so:

```rust
#[get("/earth?<continent>&<country>")]
fn earth_country(continent: String, country:String){
    format!("You've landed in earth at {} continent and {} country", continent, country)
}

```

If you put `http://localhost:8000/earth?continent=asia&country=philippines` in your RRL you'll see that it'll output: "You've landed in earth at asia continent and philippines country"


### Optional Parameters

Query parameters can be set to be optional by doing so:

```rust
#[get("/mars?<continent>&<country>&<state>")]
fn mars_country_state(continent: &RawStr, country: &RawStr, state: Option<&RawStr>) -> String{
    if let Some(state) = state{
        format!("You've landed in mars at {} continent and {} country and {} state", continent, country, state)
    }else{
        format!("You've landed in mars at {} continent and {} country", continent, country)
    }
}
```

Having the state as option, calling the path without it will still work as expected:`http://localhost:8000/mars?continent=water&country=melon`.

### Cookies

Cookies are important, they can be use for authentication and authorization, improve product recommendation, etc.

In rocket, we can set our own cookie and control it on our own accord. 

```rust
use rocket::http::{Cookies, Cookie};
#[get("/resource/<id>")]
fn set_cookie(id: String, cookie: Cookies) -> String{
    cookie.add_private(Cookies::new("user_id", id));
    String::from("Cookies was set");
}

#[get("/user")]
fn get_user(cookie: Cookies) -> String{
    format!("User ID is: {}", cookie.get_private("user_id").unwrap().value());
}

```

Now try to first go to `/resource/<id>` then go to `/user` and vice versa. You'll see how amazing it is.


#### Types of Cookies

It would be helpful for you to know about 2 types of important cookie.

- Session
  - Stay for as long as the tab is alive
- Persistent
  - Stay until the end of the browser's life.


### Format

We can specify the specific format we want to only accept or respond in our function by adding `format` inr our function attribute:

```rust
#[post("/", format = "application/json")]
fn do_something(){
    //...
}
```

In this case, whether there's a post request, in order for the `do_something()` be invoke, it should have a `content-type` of `application/json` (you can also you `json` if you want to).

### Body Data

If you want to process the data of the a request, we can do this by adding `data` to the function's attribute:

```rust

#[post("/", format="json", data="<param>")]
fn accept_something(param: T){
    // do something
}
```

Now the type `T` should be something that implements the `FromData` trait.

#### Forms Processing

If we want to process the values passed by a form of your front-end we need to make use of the the `form` type.

```rust

use rocket::request::Form;

#[derive(FromForm)]
struct UserData{
    name: String,
    age: u32,
    email: String,
}

#[post("/doSomething", data = "<tas>")]
fn somethingNew(data: Form<UserData>){
    // do something
}
```
#### Linent Parsing

FromForm requires that the number of field is the same as the field to be submitted. This means that if ever that there was additional or missing field, it would fail. If want to still continue if there's still a missing field, we can make use of linent parsing

```rust
use rocket::request::LinentForm

#[derive(FromForm)]
struct SomeData{
    // some data
}


#[post("/", data="<param>")]
fn do_something(my_data: LinentForm<SomeData>){
    // do something with the data
}

```

#### Field Renaming

Maybe you would want to rename a field of your struct. We can do this by adding `#[form(field = "value")]`

```rust

#[derive(FromForm)]
struct Something{
    #[form(fiend = "secret")]
    api_type: String
}
```

#### Sample Program for POST

**main.rs**
```rust
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::http::Cookies;

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




fn main(){
    let server = rocket::ignite();
    let server = server.mount("/", routes![index, set_cookie, get_cookie, form, do_something]);
    let server = server.launch();
}
```

**static/index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Web</title>
</head>
<body>
    <form method="POST" action="/doSomething">
        <input type="text" name="name" placeholder="Name"> <br>
        <input type="text" name="age" placeholder="Age"> <br>
        <input type="text" name="email" placeholder="Email"> <br>
        <input type="password" name="password" placeholder="Password"> <br>
        <input type="submit" value="Submit">
    </form>
</body>
</html>
```

#### Field Validation

If you want to validate submissions in your form. We can make use of `FromFormValue`

```rust

use rocket::http::RawStr;
use rocket::request::FromFormValue;

struct UserAge(usize);

impl<'v> FromFormValue<'v> for UserAge{
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<AdultAge, &'v RawStr> {
        math form_value.parse::usize(){
            Ok(age) if age >= 21 => {
                Ok(AdultAge(age))
            },
            _ => Err(form_value),
        }
    }
}

#[derive(FromForm)]
struct Person{
    age: UserAge,
}
```

Each time you've assigned a value to Person, since its type implements the trait `FromFormValue` it would call the `from_form_value` function to validate it.

### Handling JSON

In order for you to handle JSON, you would need to depend on serde. So add it into your dependencies.

We can make use of serde in this following example:

```rust
use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
struct Task{
    description: String,
    complete: bool,
}

#[post("/todo", data = "<task>")]
fn new(task: Json<Task>){
    // ...
}

```

### Streaming

If you want to handle incoming data directly. We can do this by using `rocket::Data`

```rust

use rocket::Data;

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, std::io::Error>{
    data.stream_to_file("/tmp/upload.txt").map(|n| n.to_string())
}

```

### Error Catchers

Routing my fail and sometimes, we want to catch the failure on our own. An example might be if we want to catch ourselves the `404 Not Found` error:

```rust
use rocket::Request;

#[catch(404)]
fn not_found(req: &Request) -> String{
    // do something
}

fn main(){
    rocket::ignite().register(catchers![not_found]);
}

```

What you see now in our main function is that we have use a new method called `register` which, in rocket sense, registers paths for errors defined in their attribute.

## Responses

Whenever we return a value from a function we give a response to the client. The type of our return value can be anything that implements the responder trait.

Take note that a response contains an HTTP status, a header, and a body. This body may be either a fixed-sized or streaming. An example of this is String (fixed-sized) and File (Streamed Response)

