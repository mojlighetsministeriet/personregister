use diesel;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use self::schema::person;
use self::schema::person::dsl::{person as personer};

mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

#[table_name="person"]
#[derive(Serialize,Deserialize,Queryable,Insertable,Debug)]
pub struct Person {
    pub uuid: String,
    pub namn: String,
    pub pers_nr: Option<String>,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub street: Option<String>,
    pub post_nr_city: Option<String>,
    }

impl Person {
    pub fn get(id: Uuid, conn: &MysqlConnection) -> Result<Person,diesel::result::Error> {
        let pers_id = id.hyphenated().to_string();
        let result = personer.find(&pers_id).limit(1).get_result::<Person>(conn);
        println!("Got id {:?}, found {:?}",pers_id, result);
        return result;
    }

}

/*
#[derive(Insertable)]
#[table_name="medlem"]
pub struct NewMedlem<'a> {
    pub uuid: &'a str,
    pub namn: &'a str,
    pub persNr: &'a str,
    pub mail: &'a str,
    pub phone: &'a str,
    pub street: &'a str,
    pub postNrCity: &'a str,
}
*/
