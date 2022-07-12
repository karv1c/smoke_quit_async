

use db::*;
use diesel::result::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::time::{SystemTime, Duration};
use std::{thread, fs};
use diesel::r2d2::{ConnectionManager,Pool, PooledConnection};
use diesel::pg::PgConnection;
#[macro_use]
extern crate diesel;
extern crate dotenv;

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
use crate::models::Fact;
pub mod schema;
pub mod models;
pub mod db;
pub mod modules;
extern crate tera;
use tera::{Context, Tera};

#[macro_use]
extern crate lazy_static;
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
    fact: Fact
}

/* fn main() {
    /* let pool = build_pool_connection();
    //let pool = pool.clone();
    let username = "Victor54".to_owned();
    let pass = "pass";
    let pool1 = pool.clone();
    thread::spawn(move || {
        let conn = pool.get().unwrap();
        //new_attempt(&conn, 23);
        //new_user(&conn, username, pass);
    }).join().unwrap();

    let pool2 = pool.clone(); */
    
} */
static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Pass: <input type=\"text\" name=\"password\"><br><input type=\"submit\"></body></html>";
static MISSING: &[u8] = b"Missing field";
static NOTNUMERIC: &[u8] = b"Number field is not numeric";

async fn handler(req: Request<Body>,pool: Pool<ConnectionManager<PgConnection>>) -> Result<Response<Body>, hyper::Error> {
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
                let (user, fact) = thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    let user = get_user_id_with_session(&conn, &session_id.unwrap().as_ref());
                    let fact = get_random_fact(&conn);
                    (user, fact)
                }).join().unwrap();
                let user2 = user.unwrap();
                let fact_init = fact.unwrap();
                let init = Init { time: SystemTime::now().duration_since(user2.stopped).unwrap(), attempts: user2.attempts, name: user2.username, fact: fact_init };
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
            let session_id = get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            match session_id {
                Some(id) => {
                    if id != "null".to_owned() {
                        match thread::spawn(move || {
                            let conn = pool.get().unwrap();
                            get_random_fact(&conn)
                        }).join() {
                            Ok(fact_result) => {
                                match fact_result {
                                    Ok(fact) => {
                                        let body = serde_json::to_string(&fact);
                                        match body {
                                            Ok(result) => {
                                                return Ok(Response::builder()
                                                .status(StatusCode::OK)
                                                .body(result.into())
                                                .unwrap())
                                            }
                                            Err(_) => todo!(),
                                        }
                                    },
                                    Err(_) => todo!(),
                                }
                            },
                            Err(_) => todo!(),
                        }
                    } else {todo!()}
                },
                None => todo!(),
            }
            /* let x = session_id.clone();
            if x.is_some() && x.unwrap() != "null".to_owned() {
                let fact = thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    let fact = get_random_fact(&conn);
                    fact
                }).join().unwrap();
                let fact_init = fact.unwrap();
                let p = serde_json::to_string(&fact_init);
                return Ok(Response::builder()
                .status(StatusCode::OK)
                .body(p.unwrap().into())
                .unwrap())
            } 
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap()) */
        },
        (&Method::GET, "/newattempt") => {
            let session_id = get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() && x.unwrap() != "null".to_owned() {
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    let user = get_user_id_with_session(&conn, &session_id.unwrap().as_ref()).unwrap();
                    new_attempt(&conn, user.id);
                }).join().unwrap();
                return Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
            } 
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
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
        /* (&Method::POST, "/") => {
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
                let mut ctx = Context::new();
                ctx.insert("user", &user.unwrap().username);
                let rendered = TEMPLATES.render("user.html", &ctx).unwrap();
                return Ok(Response::builder()
                .status(StatusCode::OK)
                .body(rendered.into())
                .unwrap())
            } 
            
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(template.into())
                .unwrap())}, */
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
        (&Method::GET, "/get") => {
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };
            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();
            let page = if let Some(p) = params.get("page") {
                p
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };
            let body = format!("You requested {}", page);
            Ok(Response::new(body.into()))
        }
        (&Method::GET, "/app.js") => {
            let mut f = File::open("app.js").await.unwrap();
            let mut source = Vec::new();
            f.read_to_end(&mut source).await;
            Ok(Response::builder()
            .status(StatusCode::OK)
            .body(source.into())
            .unwrap())}, 
        (&Method::OPTIONS, "/register") => {
                Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Method", "GET, POST")
                .header("access-control-Allow-headers", "content-type")
                .body(Body::empty())
                .unwrap())},
        (&Method::OPTIONS, "/login") => {
            Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Method", "GET, POST")
            .header("access-control-Allow-headers", "WWW-Authenticate,authorization, content-type")
            .body(Body::empty())
            .unwrap())},
        (&Method::OPTIONS, "/") => {
            Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Method", "GET, POST")
            .header("access-control-Allow-headers", "content-type")
            .body(Body::empty())
            .unwrap())}, 
/*         (&Method::OPTIONS, "/redirect") => {
            println!("3");
            println!("{:?}",req);
            //Ok(Response::new(template.into()))},
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Method", "GET, POST")
                .header("access-control-Allow-headers", "content-type")
                .body(Body::empty())
                .unwrap())}, */
        _ => {
            println!("{:?}",req);
            Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap())},
            
    }
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    /* let mut headers = Headers::new();
    headers.set(
    AccessControlAllowOrigin::Any
    ); */
    let pool = build_pool_connection();
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        let pool = pool.clone();
        
        /* let new_service = move |req| {
            let conn = pool.get().unwrap();
            service_fn(move |req| handler(req, &conn))
        }; */
        async move { Ok::<_, Infallible>(service_fn(move |req| handler(req, pool.clone()))) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
