extern crate iron;

mod router;

mod controller;
mod model;
mod helper;

use application::router::*;
use self::iron::prelude::Iron;

pub fn run() {
    Iron::new(router::all()).http("localhost:3000").unwrap();
}