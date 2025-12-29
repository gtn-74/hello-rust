use sqlx::PgPool;
use crate::modules::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct UserService;

impl UserService {
    // !Result<Vec<User>, String> success時:User型をResultする。failed時:StringをResultする。
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, String> {
        UserRepository::find_all(pool)
            .await
            .map_err(|e| format!("Failed to fetch users: {}", e))
    }

    pub async fn create_user(pool: &PgPool, user: User) -> Result<User, String> {
        UserRepository::create(pool, &user)
            .await
            .map_err(|e| format!("Failed to create user: {}", e))
    }
}
