#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::io;

// mod api;
mod moc_bdd;
mod todo;

#[get("/")]
fn index() -> io::Result<rocket::response::NamedFile> {
    NamedFile::open("statics/index.html")
}

fn main() {
    rocket::ignite()
        .manage(moc_bdd::Todos::init())
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("statics"))
        // .mount("/api", api::get_api_routes())
        .launch();
}
