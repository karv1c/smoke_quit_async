use crate::models::{Fact, NewSession, NewUser, Session, User, Achievement};
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error;
use dotenv::dotenv;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::env;
use std::thread;
use std::time::{Duration, SystemTime};
//use crate::schema::sessionsinfo;
use crate::schema::facts::dsl::{body, facts, id as fact_db_id, link, title};
use crate::schema::sessionsinfo::dsl::{sessionid, sessionsinfo, userid};
use crate::schema::achievements::dsl::achievements;
use crate::schema::users::dsl::{
    attempts, id as user_db_id, stopped, username as user_db_name, users,
};
pub enum ErrorType {
    WrongPassword,
    NoCookie
}

pub fn build_pool_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder().max_size(15).build(manager).unwrap()
}

pub fn new_user(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    username: &str,
    pass: &str,
) -> Result<User, Error> {
    let salt = generate_string();
    let hashpass = generate_hash(&pass, &salt);
    let new_user = NewUser {
        username,
        hashpass: &hashpass,
        salt: &salt,
    };

    diesel::insert_into(users)
        .values(&new_user)
        //.execute(connection)
        .get_result::<User>(conn)
}
pub fn new_attempt(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    id: i32,
) -> Result<User, Error> {
    diesel::update(users.find(id))
        .set((attempts.eq(attempts + 1), stopped.eq(SystemTime::now())))
        .get_result::<User>(conn)
}
pub fn get_user(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    id: i32,
) -> Result<User, Error> {
    users.find(id).get_result::<User>(conn)
}

pub fn get_random_fact(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Fact, Error> {
    let count: i64 = facts.count().get_result(conn)?;
    let random = rand::thread_rng().gen_range(0..=count) as i32;
    facts.find(random).get_result::<Fact>(conn)
}

pub fn check_password(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
    id: i32,
    pass: &str,
) -> Result<User> {
    let searched_user: User = users.find(id).get_result::<User>(connection)?;
    let salt = &searched_user.salt;
    if searched_user.hashpass == generate_hash(pass, &salt) {
        return Ok(searched_user);
    }
    Err(anyhow::Error::msg("Wrong password"))
}
fn generate_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

fn generate_hash(pass: &str, salt: &str) -> String {
    let salted_pass = format!("{}{}", salt, pass);
    let mut hasher: sha2::Sha256 = Sha256::new();
    hasher.update(salted_pass.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn new_session(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    user: User,
) -> Result<Session, Error> {
    let session_id = generate_string();
    let expire_time = SystemTime::now() + Duration::new(7 * 24 * 3600, 0);
    let session = NewSession {
        sessionid: &session_id,
        userid: user.id,
        expire: expire_time,
    };

    diesel::insert_into(sessionsinfo)
        .values(session)
        //.execute(connection)
        .get_result::<Session>(conn)
}

pub fn get_user_id_with_session(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    session_id: &str,
) -> Result<User, Error> {
    let id: i32 = sessionsinfo
        .filter(sessionid.eq(session_id))
        .select(userid)
        .first(conn)?;
    get_user(conn, id)
}
pub fn get_user_id_with_namepass(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    username: &str,
    password: &str,
) -> Result<User> {
    let id: i32 = users
        .filter(user_db_name.eq(username))
        .select(user_db_id)
        .first(conn)?;
    //let user = get_user(conn, id);
    let user = check_password(conn, id, password)?;
    Ok(user)
}

pub fn get_achievements(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Achievement>> {
    Ok(achievements.load::<Achievement>(conn)?)
}

pub fn remove_session(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    session_id: String,
) -> Result<(), Error> {
    diesel::delete(sessionsinfo.filter(sessionid.eq(session_id))).execute(conn)?;
    Ok(())
}

pub fn new_thread<F: 'static + std::marker::Send>(
    pool: Pool<ConnectionManager<PgConnection>>,
    f: F,
) {
    thread::spawn(move || {
        let conn = pool.get().unwrap();
        f;
    })
    .join()
    .unwrap();
}
