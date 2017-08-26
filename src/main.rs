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

mod api_error;
mod person;
mod db;

use rocket::Request;
use rocket_contrib::{JSON, UUID, SerdeError};
use person::{Person, ClientPerson};
use api_error::ApiError as ApiErrors;

type GetPersonReply = Result<JSON<Person>,ApiErrors>;
type ParsedJsonPerson = Result<JSON<ClientPerson>, SerdeError>;

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
    let result = Person::get(*id, &conn)?;
    Ok(JSON(result))
}


#[post("/person", format = "application/json", data = "<persondata>")]
fn create_person(persondata: ParsedJsonPerson, conn: db::Conn) -> GetPersonReply {
    let persondata = match persondata {
        Ok(data) => data,
        Err(_) => return Err(ApiErrors::InvalidJsonError)   };

    let new_name = match persondata.0.namn {
        Some(ref name) if name.len() > 0 => persondata.0.namn.clone(),
        None                             => return Err(ApiErrors::EmptyNameError),
        _                                => return Err(ApiErrors::EmptyNameError)   };

    let result = Person::create(persondata.0, new_name.unwrap(), &conn)?;
    Ok( JSON(result) )    
}


#[put("/person/<id>", format = "application/json", data = "<persondata>")]
fn update_person(id: UUID, persondata: ParsedJsonPerson, conn: db::Conn) -> GetPersonReply {
    let persondata = match persondata { 
        Ok(data)  => data,
        Err(_) => return Err(ApiErrors::InvalidJsonError)   };

    let result = ClientPerson::update(persondata.0, *id, &conn)?;
    Ok( JSON(result) )
}

#[allow(unused_variables)] 
// Someday there will be more handling here
#[error(404)] 
fn not_found(req: &Request) -> ApiErrors {
    ApiErrors::InvalidRequestError
}


fn main() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index,person,create_person,update_person])
        .launch();
}
