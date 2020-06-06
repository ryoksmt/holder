extern crate iron;
extern crate rustc_serialize;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use rustc_serialize::json;
use urlencoded::UrlEncodedQuery;
          #[derive(RustcEncodable)]
pub struct Letter {
    title: String,
    message: String
}

pub fn hello(req: &mut Request) -> IronResult<Response> {
    let letter = Letter{
        title: "PPAP".to_string(),
        message: "i have a pen, i have an apple.".to_string()
    };
    let maps = req.get_ref::<UrlEncodedQuery>();
    println!("{:#?}",maps.unwrap().get("hoge").unwrap()[0]);
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => println!("Parsed GET request query string:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };
    let payload = json::encode(&letter).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

