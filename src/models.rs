//use diesel::{sql_types::Timestamp, data_types::PgTimestamp};
use super::schema::{facts, sessionsinfo, users};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Fact {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub link: Option<String>,
}
#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Achievement {
    pub id: i32,
    pub body: String,
    pub duration: i32,
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
