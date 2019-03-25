#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate postgres;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use rocket::response::{Redirect, status};
use rocket::Response;
use rocket::request::Form;

#[derive(FromForm, Debug)]
struct Url {
    path: String,
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("message", "Hello templates");
    Template::render("index", &context)
}

#[post("/new_url", data="<url>")]
fn new_url(url: Form<Url>) -> Redirect {
    println!("{:?}", url);
    Redirect::to("/")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, new_url])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}