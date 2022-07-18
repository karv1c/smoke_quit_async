use crate::db::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::time::{Duration, SystemTime};
use std::{fs, thread};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use hyper::{Body, Method, Request, Response, StatusCode};

use crate::models::{Achievement, Fact};
use crate::modules::cookieparser::{get_sessionid_from_cookie_s, parse_cookie_s};

#[derive(Serialize, Deserialize)]
pub struct FormRequest {
    pub username: String,
    pub password: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Init {
    time: Duration,
    attempts: i32,
    name: String,
    fact: Fact,
    achievements: Vec<Achievement>,
}
pub async fn handler(
    req: Request<Body>,
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::POST, "/") => {
            //let x = req.;
            //let x =&req.;
            println!("{:?}", req);
            let template = fs::read_to_string("index.html").unwrap();
            let session_id =
                get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() && x.unwrap() != "null".to_owned() {
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    get_user_id_with_session(&conn, &session_id.unwrap().as_ref());
                })
                .join()
                .unwrap();
                let template = fs::read_to_string("user.html").unwrap();
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(template.into())
                    .unwrap());
            }

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(template.into())
                .unwrap())
        }
        (&Method::GET, "/initialize") => {
            println!("{:?}", req);
            let session_id =
                get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() && x.unwrap() != "null".to_owned() {
                let (user, fact, achievements) = thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    let user = get_user_id_with_session(&conn, &session_id.unwrap().as_ref());
                    let fact = get_random_fact(&conn);
                    let achievements = get_achievements(&conn);
                    (user, fact, achievements)
                })
                .join()
                .unwrap();
                let user2 = user.unwrap();
                let fact_init = fact.unwrap();
                let mut achievements_init = achievements?;
                achievements_init.sort_by_key(|k| k.id);
                let init = Init {
                    time: SystemTime::now().duration_since(user2.stopped).unwrap(),
                    attempts: user2.attempts,
                    name: user2.username,
                    fact: fact_init,
                    achievements: achievements_init,
                };
                let p = serde_json::to_string(&init);
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(p.unwrap().into())
                    .unwrap());
            }
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
        }
        (&Method::GET, "/fact") => {
            let fact = thread::spawn(move || {
                let conn = pool.get().unwrap();
                get_random_fact(&conn)
            })
            .join()
            .unwrap()?;
            let body = serde_json::to_string(&fact)?;
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .body(body.into())
                .unwrap());
        }
        (&Method::GET, "/newattempt") => {
            match get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie"))) {
                Some(session_id) => {
                    if session_id != "null".to_owned() {
                        thread::spawn(move || {
                            let conn = pool.get().unwrap();
                            let user = get_user_id_with_session(&conn, &session_id.as_ref())?;
                            new_attempt(&conn, user.id)
                        })
                        .join()
                        .unwrap()?;
                        return Ok(Response::builder()
                            .status(StatusCode::OK)
                            .body(Body::empty())
                            .unwrap());
                    } else {
                        return Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())
                            .unwrap());
                    }
                }
                None => {
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap())
                }
            }
        }
        (&Method::GET, "/logout") => {
            let session_id =
                get_sessionid_from_cookie_s(parse_cookie_s(req.headers().get("cookie")));
            let x = session_id.clone();
            if x.is_some() {
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    remove_session(&conn, x.unwrap());
                })
                .join()
                .unwrap();
            }
            Ok(Response::builder()
                .status(StatusCode::PERMANENT_REDIRECT)
                .header("Set-cookie", "sessionid=null")
                .header("location", "/")
                .body(Body::empty())
                .unwrap())
        }
        (&Method::POST, "/login") => {
            println!("{:?}", req);
            let b = hyper::body::to_bytes(req).await?;
            let form_request: FormRequest = serde_json::from_slice(b.as_ref()).unwrap();
            let conn = pool.get().unwrap();
            let result = thread::spawn(move || {
                match get_user_id_with_namepass(
                    &conn,
                    &form_request.username,
                    &form_request.password,
                ) {
                    Ok(user) => {
                        let session_id = new_session(&conn, user).unwrap().sessionid;
                        Some(session_id)
                    }
                    Err(_) => None,
                }
            })
            .join()
            .unwrap();
            match result {
                Some(session_id) => {
                    let body = "{\"message\": \"done\"}";
                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Set-cookie", format!("sessionid={}", session_id))
                        .body(body.into())
                        .unwrap());
                }
                None => {
                    let body = "{\"message\": \"Wrong Username or Password\"}";
                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(body.into())
                        .unwrap());
                }
            }
        }
        (&Method::POST, "/register") => {
            let b = hyper::body::to_bytes(req).await?;

            let form_request: FormRequest = serde_json::from_slice(b.as_ref()).unwrap();

            let conn = pool.get().unwrap();
            let result = thread::spawn(move || {
                match new_user(&conn, &form_request.username, &form_request.password) {
                    Ok(user) => {
                        let session_id = new_session(&conn, user).unwrap().sessionid;
                        Some(session_id)
                    }
                    Err(_) => None,
                }
            })
            .join()
            .unwrap();
            match result {
                Some(session_id) => {
                    let body = "{\"message\": \"done\"}";
                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Set-cookie", format!("sessionid={}", session_id))
                        .body(body.into())
                        .unwrap());
                }
                None => {
                    let body = "{\"message\": \"Username is not available\"}";
                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(body.into())
                        .unwrap());
                }
            }
        }
        (&Method::GET, "/app.js") => {
            let mut f = File::open("app.js").await.unwrap();
            let mut source = Vec::new();
            f.read_to_end(&mut source).await;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(source.into())
                .unwrap())
        }
        (&Method::GET, "/register_form.js") => {
            let mut f = File::open("register_form.js").await.unwrap();
            let mut source = Vec::new();
            f.read_to_end(&mut source).await;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(source.into())
                .unwrap())
        }
        _ => {
            println!("{:?}", req);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap())
        }
    }
}
