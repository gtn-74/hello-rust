use sqlx::PgPool;
use crate::modules::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(include_str!("sql/find_all_users.sql"))
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(include_str!("sql/create_user.sql"))
            .bind(&user.user_name)
            .bind(&user.email)
            .fetch_one(pool)
            .await
    }
}
