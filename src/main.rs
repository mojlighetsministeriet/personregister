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

use rocket_contrib::{JSON, UUID};
use person::{Person, ClientPerson};

type GetPersonReply = Option<JSON<Person>>;
type JsonPerson = JSON<ClientPerson>;


#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      GET /person/<id>

          retrieves the content for the person with id `<id>`

      PUT /person/<id> 

          Updates properties of the person identified by the 
          supplied id with the given values. Properties to be
          updated should be sent as json encoded property:value
          pairs.
    "
}

#[get("/person/<id>")]
fn person(id: UUID, conn: db::Conn) -> GetPersonReply {
    match Person::get(*id, &conn) {
        Ok(found) => Some( JSON( found ) ),
        Err(_)    => None 
    }
}

#[post("/person/<id>", format = "application/json", data = "<persondata>")]
fn create_person(id: UUID, persondata: JSON<ClientPerson>, conn: db::Conn) -> () { // GetPersonReply {
   /* match Person::create(persondata.0, &conn) {
        Ok(found) => Some( JSON( found ) ),
        Err(_)    => None 
    } */
}


#[put("/person/<id>", format = "application/json", data = "<persondata>")]
fn update_person(id: UUID, persondata: JSON<ClientPerson>, conn: db::Conn) -> GetPersonReply {
    match ClientPerson::update(persondata.0, *id, &conn) {
        Ok(found) => Some( JSON( found ) ),
        Err(_)    => None 
    }
}


fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index,person,create_person,update_person])
        .launch();
}
