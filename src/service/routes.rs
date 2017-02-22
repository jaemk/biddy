///! Route definitions
///!
use router::Router;

use handlers::{Handlers};


pub fn mount(router: &mut Router, handlers: Handlers) {

    router.get("/hello", handlers.hello, "hello");
    router.get("/info", handlers.info, "info");

    router.post("/login", handlers.login, "login");
    router.post("/logout", handlers.logout, "logout");

    router.get("/items", handlers.items, "items");
    //router.get("/users", handlers.users, "users");
    //router.post("/msg", handlers.post_msg , "post_msg");
    //router.get("/msg", handlers.get_msg, "get_msg");
}
