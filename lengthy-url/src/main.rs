#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate postgres;

use serde::{Serialize, Deserialize};
// use serde_json::json::Json;

use rocket_contrib::templates::Template;

use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;

use rocket::response::{Redirect, status};
use rocket::http::uri::Uri;
use rocket::Response;
use rocket::request::Form;
use rocket::State;
use std::sync::RwLock;
use rocket::response::content;

use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[derive(FromForm, Debug, Deserialize)]
struct Url {
    path: String,
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("message", "Hello templates");
    Template::render("index", &context)
}

#[get("/l/<encoded>")]
fn long_url(encoded: String, state: State<RwLock<HashMap<String, String>>>) -> Redirect {
    println!("Looking for: {:?}", encoded);
    let lock = state.inner();
    let urls = lock.read().unwrap();
    let short = urls.get(&encoded);
    println!("Known: {:?}", urls);
    println!("Looking for: {:?}", encoded);
    match short {
        Some(redir) => {
            println!("Matched a long url, redirecting to {:?}", redir.clone());
            return Redirect::to(redir.clone());
        },
        None => Redirect::to("/")
    }
}

#[derive(Serialize)]
struct UrlResponse {
    url: String
}

#[post("/new_url", data="<url>")]
fn new_url(url: Json<Url>, state: State<RwLock<HashMap<String, String>>>) -> Json<UrlResponse> {
    println!("{:?}", url);
    let lock = state.inner();
    let mut urls = lock.write().unwrap();
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(1024)
        .collect();
    println!("Generated: {:?}", &chars);
    urls.insert(chars.clone(), url.path.to_owned());

    let to_return = UrlResponse{url: chars.clone()};
    return Json(to_return);
}

fn main() {
    let mut urls: HashMap<String, String> = HashMap::new();
    let lock = RwLock::new(urls);

    rocket::ignite()
        .mount("/", routes![index, new_url, long_url])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .manage(lock)
        .launch();
}