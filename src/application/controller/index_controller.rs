extern crate iron;
extern crate router;
extern crate rustc_serialize;

use self::iron::prelude::*;
use self::router::Router;
use self::rustc_serialize::json;
use application::model::User;

pub struct IndexController;

impl IndexController {
	pub fn index(_: &mut Request) -> IronResult<Response> {
		let user = User::get_by_id(1);
		let encoded_user = json::encode(&user).unwrap();
	    Ok(Response::with((iron::status::Ok, encoded_user)))
	}
}