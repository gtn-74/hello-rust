#[macro_use]
extern crate rocket;

mod controller;
mod modules;
mod repositories;
mod services;

use rocket::launch;
use sqlx::PgPool;

#[launch]
async fn rocket() -> _ {
    // .envファイルから環境変数を読み込む
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .manage(pool)
        .mount("/", routes![controller::user::index])
        .mount("/api", routes![controller::user::get_users, controller::user::create_user])
}
