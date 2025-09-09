use diesel::{QueryDsl, ExpressionMethods, SelectableHelper, OptionalExtension, BoolExpressionMethods, update};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Deserialize;
use crate::models::User;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::schema::issued_jwt_tokens::dsl::issued_jwt_tokens;
use crate::schema::issued_jwt_tokens::{device_id, revoked, user_id};
use crate::schema::users::{e_mail, username};
use crate::schema::users::dsl::users;

#[derive(Deserialize)]
pub struct AuthInput {
    pub login: String,
    pub password: String,
    pub device_id: String
}


impl AuthInput {

    fn is_username(&self) -> bool {
        !self.login.contains(['.', '@'])
    }

    pub async fn find_user(&self, conn: &mut AsyncPgConnection) -> Result<Option<User>, diesel::result::Error> {

        if self.is_username() {
            users
                .filter(username.eq(&self.login))
                .select(User::as_select())
                .first::<User>(conn)
                .await
                .optional()
        } else {
            users
                .filter(e_mail.eq(&self.login))
                .select(User::as_select())
                .first::<User>(conn)
                .await
                .optional()
        }

    }

    pub fn verify_password(&self, hashed_password: &str, argon2: Argon2) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(&hashed_password)?;
        Ok(argon2.verify_password(&self.password.as_bytes(), &parsed_hash).is_ok())
    }

    pub async fn is_authorized(&self, conn: &mut AsyncPgConnection, user: &User) -> Result<(), diesel::result::Error > {

        update(issued_jwt_tokens)
            .filter(user_id.eq(&user.id).and(device_id.eq(&self.device_id)))
            .set(revoked.eq(true))
            .execute(conn)
            .await?;

        Ok(())

    }

    

}
