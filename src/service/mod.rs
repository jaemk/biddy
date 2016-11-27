use std::sync::{Arc, Mutex};

use super::iron::prelude::*;
use super::iron::status;
use super::router::Router;
use super::env_logger;
use super::logger::Logger;

use super::rustc_serialize::json;

use super::r2d2::{Config, Pool};
use super::r2d2_postgres::{PostgresConnectionManager, TlsMode};

use super::sql;
use super::handlers::{Handlers};
use super::middleware::{InfoLog, SessionWatch};
use super::sessions::{SessionStore};

pub fn start() {
    // setup db connection pool
    let db_url = "postgresql://biddy:biddy@localhost";
    let db_mgr = PostgresConnectionManager::new(db_url, TlsMode::None).expect("connection fail");
    let db_pool = Pool::new(Config::default(), db_mgr).expect("pool fail");
    println!(">> Connected to db!");

    // setup session store access
    let session_store = Arc::new(Mutex::new(SessionStore::new()));
    let session_watch = SessionWatch::new(session_store.clone());
    println!(">> Session store created");

    // setup general loggers
    env_logger::init().unwrap();
    let (log_before, log_after) = Logger::new(None);

    // initialize handler manager with external resources
    let handlers = Handlers::new(db_pool, session_store);

    let mut router = Router::new();

    router.get("/hello", handlers.hello, "hello");
    router.get("/users", handlers.users, "users");
    router.get("/login", handlers.login, "login");
    router.post("/msg", handlers.post_msg , "post_msg");

    let mut chain = Chain::new(router);
    chain.link_before(log_before);       // general logger
    chain.link_before(InfoLog);          // custom request-info log
    chain.link_around(session_watch);
    chain.link_after(log_after);         // general logger

    Iron::new(chain).http("localhost:8000").unwrap();
}
