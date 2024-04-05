use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
    pub pw: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub pseudo: &'a str,
    pub pw: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}