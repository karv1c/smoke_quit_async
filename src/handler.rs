
use crate::db::*;
use crate::schema::achievements;
use diesel::result::Error;
use tera::Tera;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::time::{SystemTime, Duration};
use std::{thread, fs};
use diesel::r2d2::{ConnectionManager,Pool, PooledConnection};
use diesel::pg::PgConnection;


use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use url::form_urlencoded;
use std::collections::HashMap;
use anyhow::Result;

use crate::modules::cookieparser::{parse_cookie_s, get_sessionid_from_cookie_s};
use crate::models::{Fact, Achievement};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
//use crate::schema::users;
//use self::models::{User, NewUser};
//use self::schema::users::dsl::{users, attempts, stopped};
#[derive(Serialize, Deserialize)]
pub struct ReqRequest {
    pub username: String,
    pub password: String
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Init {
    time: Duration,
    attempts: i32,
    name: String,
    fact: Fact,
    achievements: Vec<Achievement>
}
pub async fn handler(req: Request<Body>,pool: Pool<ConnectionManager<PgConnection>>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::POST, "/") => {
            //let x = req.;
            //let x =&req.;
            println!("{:?}",req);
            let template = fs::read_to_string("index.html").unwrap();
            let session_id = get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() && x.unwrap() != "null".to_owned() {
                let user = thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    let user = get_user_id_with_session(&conn, &session_id.unwrap().as_ref());
                    user
                }).join().unwrap();
                /* let mut ctx = Context::new();
                ctx.insert("user", &user.unwrap().username);
                let rendered = TEMPLATES.render("user.html", &ctx).unwrap(); */
                let p =  "{\"authorized\": 1}";
                let template = fs::read_to_string("user.html").unwrap();
                return Ok(Response::builder()
                .status(StatusCode::OK)
                //.body(p.into())
                .body(template.into())
                .unwrap())
            } 
             /*  */
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(template.into())
                .unwrap())},
        (&Method::GET, "/initialize") => {
            println!("{:?}" ,req);
            let session_id = get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() && x.unwrap() != "null".to_owned() {
                let (user, fact, achievements) = thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    let user = get_user_id_with_session(&conn, &session_id.unwrap().as_ref());
                    let fact = get_random_fact(&conn);
                    let achievements = get_achievements(&conn);
                    (user, fact, achievements)
                }).join().unwrap();
                let user2 = user.unwrap();
                let fact_init = fact.unwrap();
                let mut achievements_init = achievements?;
                achievements_init.sort_by_key(|k| k.id);
                let init = Init { time: SystemTime::now().duration_since(user2.stopped).unwrap(), attempts: user2.attempts, name: user2.username, fact: fact_init, achievements: achievements_init};
                let p = serde_json::to_string(&init);
                return Ok(Response::builder()
                .status(StatusCode::OK)
                .body(p.unwrap().into())
                .unwrap())
            } 
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
        },
        (&Method::GET, "/fact") => {
            let fact = thread::spawn(move || {
                let conn = pool.get().unwrap();
                get_random_fact(&conn)
                }).join().unwrap()?;
            let body = serde_json::to_string(&fact)?;
            return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(body.into())
            .unwrap())
        },
        (&Method::GET, "/newattempt") => {
            match get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie"))) {
                Some(session_id) => {
                    if session_id != "null".to_owned() {
                        thread::spawn(move || {
                            let conn = pool.get().unwrap();
                            let user = get_user_id_with_session(&conn, &session_id.as_ref())?;
                            new_attempt(&conn, user.id)
                        }).join().unwrap()?;
                        return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::empty())
                        .unwrap())
                    } else {
                        return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap())
                    }
                },
                None => {
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap())
                }
            }
        },
        (&Method::GET, "/logout") => {
            let session_id = get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() {
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    remove_session(&conn, x.unwrap());
                }).join().unwrap();
            } 
            Ok(Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header("Set-cookie", "sessionid=null")
            .header("location", "/")
            .body(Body::empty())
            .unwrap())
        }
        (&Method::POST, "/login") => {
            println!("{:?}",req);
            let b = hyper::body::to_bytes(req).await?;

            let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            println!("{:?}", &params);

            let name = if let Some(n) = params.get("username") {
                n.to_owned()
            } else {

                "null".to_owned()
            };
            let password = if let Some(n) = params.get("password") {
                n.to_owned()
            } else {
                "null".to_owned()
            };

            let conn = pool.get().unwrap();
            let mut body = String::new();
            let mut session_id = String::new();
            let result = thread::spawn(move || {
                
                match get_user_id_with_namepass(&conn, &name, &password) {
                    Ok(user) => {
                        session_id = new_session(&conn, user).unwrap().sessionid;
                        return Some(session_id)
                    },
                    Err(_) => {
                        None
                    },
                }
            }).join().unwrap();
            Ok(Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header("Set-cookie", format!("sessionid={}", result.unwrap()))
            .header("location", "/")
            .body(Body::empty())
            .unwrap())

        }
        (&Method::POST, "/register") => {

            println!("{:?}", &req);
            let b = hyper::body::to_bytes(req).await?;

            let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            println!("{:?}", &params);

            let name = if let Some(n) = params.get("username") {
                n.to_owned()
            } else {

                "null".to_owned()
            };
            let password = if let Some(n) = params.get("password") {
                n.to_owned()
            } else {
                "null".to_owned()
            };

            let conn = pool.get().unwrap();
            let mut body = String::new();
            let mut session_id = String::new();
            let result = thread::spawn(move || {
                
                match new_user(&conn, &name, &password) {
                    Ok(user) => {
                        body = format!("Hello {}, you are in db now", &name);
                        session_id = new_session(&conn, user).unwrap().sessionid;
                    },
                    Err(_) => {
                        body = format!("Name {} is already exist", &name);
                    },
                }
                
                (body, session_id)
            }).join().unwrap();
            Ok(Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header("Set-cookie", format!("sessionid={}", result.1))
            .header("location", "/")
            .body(Body::empty())
            .unwrap())

        }
        (&Method::GET, "/app.js") => {
            let mut f = File::open("app.js").await.unwrap();
            let mut source = Vec::new();
            f.read_to_end(&mut source).await;
            Ok(Response::builder()
            .status(StatusCode::OK)
            .body(source.into())
            .unwrap())}, 
        _ => {
            println!("{:?}",req);
            Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap())},
            
    }
}