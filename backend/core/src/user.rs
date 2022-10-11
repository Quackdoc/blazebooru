use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use blazebooru_models::local as lm;
use blazebooru_models::view as vm;
use blazebooru_store::models as dbm;

use super::BlazeBooruCore;

impl BlazeBooruCore {
    pub async fn create_user(&self, user: lm::NewUser<'_>) -> Result<i32, anyhow::Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|err| anyhow!("{err}"))?;

        let user = dbm::NewUser {
            name: Some(user.name.into()),
            password_hash: Some(password_hash.to_string()),
        };

        let user = self.store.create_user(&user).await?;

        Ok(user.id)
    }

    pub async fn get_user_profile(&self, user_id: i32) -> Result<Option<vm::User>, anyhow::Error> {
        let user = self.store.get_user(user_id).await?;

        Ok(user.map(vm::User::from))
    }
}
