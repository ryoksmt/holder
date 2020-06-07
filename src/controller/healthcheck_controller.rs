extern crate iron;
extern crate rustc_serialize;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use rustc_serialize::json;
use urlencoded::UrlEncodedQuery;
use crate::usecase::healthcheck_usecase::*;
use crate::entity::healthcheck_entity::Letter;
use std::alloc::handle_alloc_error;

pub fn hello(req: &mut Request) -> IronResult<Response> {
    let hcu = HealthcheckUsecase;
    HealthcheckUsecase::healthcheck(&hcu);

    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => println!("Parsed GET request query string:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };
    let payload = json::encode(&letter).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

