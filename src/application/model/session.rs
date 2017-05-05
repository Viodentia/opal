use self::User;
use uuid::Uuid;

pub struct Session {
	id : String,
	user : User,
	pets : Vec<String>
}

impl Session {
	pub fn open_new(id : i64) -> User {
		let session = Session {
			id : Uuid::new_v4(),
			user : User::get_by_id(1)
		};
		session
	}
}