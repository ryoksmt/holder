use crate::application::router::routers;
extern crate iron;
use iron::prelude::*;

pub fn server(){
    let routers = routers();
    Iron::new(routers).http("localhost:3000").unwrap();
}