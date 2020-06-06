extern crate iron;
extern crate router;

use router::Router;

use crate::ctl::healthcheck_ctl::hello;

pub fn routers() -> Router {
    let mut router = Router::new();
    router.get("/", hello, "hello");
    return router
}