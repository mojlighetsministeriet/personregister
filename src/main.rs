#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate uuid;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel_mysql;
extern crate dotenv;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;


mod person;
mod db;

use uuid::Uuid;
use rocket_contrib::{JSON, UUID};
use person::Person;


#[derive(Serialize,Deserialize)]
struct Message {
    id: Option<Uuid>,
    contents: Person
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      GET /person/<id>

          retrieves the content for the person with id `<id>`
    "
}

#[get("/person/<id>")]
fn person(id: UUID, conn: db::Conn) -> Option<JSON<Message>> {
//   let found 
    match Person::get(*id, &conn) {
        Ok(found) => Some( JSON( Message { contents: found, id: Some(*id)}) ),
        Err(_)    => None 
    }
}

fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index,person])
        .launch();
}
