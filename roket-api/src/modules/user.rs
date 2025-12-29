use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}
