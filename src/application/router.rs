extern crate router;

use application::controller::UserController;
use application::controller::IndexController;
use self::router::Router;

pub fn all() -> Router {
	let mut router = Router::new();  
	router.get("/", IndexController::index, "index");
    router.post("/users", UserController::create, "user_create");
    router.post("/users/login/:id", UserController::login, "user_login");
    router.get("/users/:id", UserController::read, "user_read");
    router.put("/users/:id", UserController::update, "user_update");
    router.delete("/users/:id", UserController::delete, "user_delete");
    router.get("/users", UserController::read_all, "user_list");
    router
}