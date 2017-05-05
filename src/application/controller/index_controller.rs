extern crate iron;
extern crate router;
extern crate serde_json;

use self::iron::prelude::*;
use self::router::Router;
use application::model::User;

pub struct IndexController;

impl IndexController {
	pub fn index(_: &mut Request) -> IronResult<Response> {
		let user = User::get_by_id(1);
		let encoded_user = serde_json::to_string(&user).unwrap();
	    Ok(Response::with((iron::status::Ok, encoded_user)))
	}
}