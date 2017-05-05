#[macro_use]
extern crate serde_derive;

mod application;

use application::*;

fn main() {
	application::run();
}