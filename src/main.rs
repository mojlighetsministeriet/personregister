#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

#[macro_use] extern crate uuid;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;



use uuid::Uuid;
use rocket_contrib::{JSON, Value, UUID};
use rocket::State;

#[derive(Serialize,Deserialize)]
struct Message {
    id: Option<Uuid>,
    contents: String
}

fn main() {
    rocket::ignite().mount("/", routes![index,person]).launch()
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
fn person(id: UUID) -> Option<JSON<Message>> {
   Some( JSON( Message {
        contents: "whatevs".to_string(),
        id: Some(*id)}) 
       )
}
