use diesel;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;


mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

use self::schema::person;
use self::schema::person::dsl::{person as personer};

type PersonResult = Result<Person,diesel::result::Error>;

#[derive(Serialize, Deserialize, //Serde json serialization
         Queryable,Insertable,Identifiable, // Diesel codegen
         Debug)]
#[table_name="person"]
#[primary_key(uuid)]
pub struct Person {
    pub uuid:         String,
    pub namn:         String,
    pub pers_nr:      Option<String>,
    pub mail:         Option<String>,
    pub phone:        Option<String>,
    pub street:       Option<String>,
    pub post_nr_city: Option<String>,
    }

impl Person {

    pub fn new_from_client(person: ClientPerson, name: String) -> (Person,Uuid) {
        let id = Uuid::new_v4(); 

        (Person { 
            uuid:         id.to_string(),
            namn:         name,
            pers_nr:      person.pers_nr,
            mail:         person.mail,
            phone:        person.phone,
            street:       person.street,
            post_nr_city: person.post_nr_city,}
        ,
        id)

    }

    pub fn get(id: Uuid, conn: &MysqlConnection) -> PersonResult {
        let pers_id = id.hyphenated().to_string();
        personer.find(&pers_id).limit(1).get_result::<Person>(conn)
    }

    pub fn create(new_person: ClientPerson, name: String, conn: &MysqlConnection) ->  PersonResult {
        let (newcomer,new_id) = Person::new_from_client(new_person,name);
        diesel::insert(&newcomer).into(person::table).execute(conn)?;
        Person::get(new_id, conn)
    }

}

#[derive(Serialize, Deserialize, 
         Identifiable, AsChangeset,
         Debug)]
#[table_name="person"]
#[primary_key(uuid)]
pub struct ClientPerson {
    pub uuid:         Option<String>,
    pub namn:         Option<String>,
    pub pers_nr:      Option<String>,
    pub mail:         Option<String>,
    pub phone:        Option<String>,
    pub street:       Option<String>,
    pub post_nr_city: Option<String>,
    }

impl ClientPerson {
    pub fn update(mut target: ClientPerson, id: Uuid, conn: &MysqlConnection) -> PersonResult {
        target.uuid = Some(id.hyphenated().to_string());
        diesel::update(person::table).set(&target).execute(conn)?;
        Person::get(id,conn)
    }
}
