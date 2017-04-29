pub struct User {
	id : i64,
	name : String,
	pets : Vec<String>
}

impl User {
	pub fn get_by_id(id : i64) -> User {
		let user = User {
			id : id,
			name : "Mojito".to_string()
		};
		user
	}
}