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

pub type Connection = std::sync::Mutex<diesel::MysqlConnection>;

#[get("/")]
fn index() -> io::Result<rocket::response::NamedFile> {
    NamedFile::open("statics/index.html")
}

fn main() {
    let conn = init::diesel_init();

    let mut_conn = std::sync::Mutex::new(conn);

    rocket::ignite()
        .manage(mut_conn)
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("statics"))
        .mount("/api", api::get_api_routes())
        .launch();
}
