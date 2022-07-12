//use diesel::{sql_types::Timestamp, data_types::PgTimestamp};
use super::schema::{sessionsinfo, users};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashpass: String,
    pub salt: String,
    pub created: SystemTime,
    pub stopped: SystemTime,
    pub attempts: i32,
}
#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub sessionid: String,
    pub userid: i32,
    pub expire: SystemTime,
}
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashpass: &'a str,
    pub salt: &'a str,
}
#[derive(Insertable)]
#[table_name = "sessionsinfo"]
pub struct NewSession<'a> {
    pub sessionid: &'a str,
    pub userid: i32,
    pub expire: SystemTime,
}
