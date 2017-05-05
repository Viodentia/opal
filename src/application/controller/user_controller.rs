extern crate iron;
extern crate router;
extern crate serde_json;

use application::model::User;
use application::helper::UserHelper;
use self::iron::prelude::*;
use self::router::Router;

pub struct UserController;

impl UserController {
	pub fn create(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn read(req: &mut Request) -> IronResult<Response> {
		let id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();
		let user = User::get_by_id(id);
		let encoded_user = serde_json::to_string(&user).unwrap();
	    Ok(Response::with((iron::status::Ok, encoded_user)))
	}
	pub fn login(req: &mut Request) -> IronResult<Response> {
		//todo
		// let login = req.extensions.get::<Router>().unwrap().find("login").unwrap().parse::<String>().unwrap();
		// let password = req.extensions.get::<Router>().unwrap().find("password").unwrap().parse::<String>().unwrap();
		
		let user = User::get_by_id(1);
		let encoded_user = serde_json::to_string(&user).unwrap();
	    Ok(Response::with((iron::status::Ok, encoded_user)))
	}
	pub fn update(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn delete(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn read_all(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
}