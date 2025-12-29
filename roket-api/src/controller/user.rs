use rocket::{
    State, get, post,
    serde::json::Json,
    http::Status,
};
use sqlx::PgPool;
use crate::modules::user::User;
use crate::services::user_service::UserService;

// GET /
#[get("/")]
pub fn index() -> &'static str {
    "Hello, Rocket!"
}

// GET /users
#[get("/users")]
pub async fn get_users(pool: &State<PgPool>) -> Result<Json<Vec<User>>, Status> {
    match UserService::get_all_users(pool.inner()).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// POST /users
#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(pool: &State<PgPool>, user: Json<User>) -> Result<Json<User>, Status> {
    match UserService::create_user(pool.inner(), user.into_inner()).await {
        Ok(created_user) => Ok(Json(created_user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
