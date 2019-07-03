#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::io;

mod api;
mod init;
mod todos;

#[get("/")]
fn index() -> io::Result<rocket::response::NamedFile> {
    NamedFile::open("statics/index.html")
}

fn main() {
    let conn = init::diesel_init();

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("statics"))
        .launch();
}
