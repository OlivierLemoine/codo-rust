#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

mod init;
mod todos;

#[get("/")]
fn index() -> rocket_contrib::serve::StaticFiles {
    StaticFiles::from("statics/index.js")
}

fn main() {
    let conn = init::diesel_init();

    rocket::ignite().mount("/", route![index]);
}