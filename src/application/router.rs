extern crate iron;
extern crate router;

use router::Router;

use crate::controller::healthcheck_controller::*;

pub fn routers() -> Router {
    let mut router = Router::new();
    router.get("/", hello, "hello");
    return router
}