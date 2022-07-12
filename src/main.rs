use db::*;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
use std::convert::Infallible;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use anyhow::Result;
pub mod schema;
pub mod models;
pub mod db;
pub mod modules;
pub mod handler;

use crate::handler::*;
extern crate tera;
//new line
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let pool = build_pool_connection();
    let make_svc = make_service_fn(move |_conn| {
        let pool = pool.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| handler(req, pool.clone()))) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
