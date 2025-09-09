use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use regex::Regex;
use serde::Deserialize;
use crate::schema::users::dsl::users;
use diesel::{
    QueryDsl, insert_into, BoolExpressionMethods, ExpressionMethods, dsl::exists, select, 
    result::{Error as DieselError}
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::error::CustomError;
use crate::models::{NewUser, User};

#[derive(Deserialize)]
pub struct UserRegistryInput {
    pub first_name: String,
    pub second_name: String,
    pub username: String,
    #[serde(rename = "e-mail")]
    pub email: String,
    pub password: String,
    role_type: Option<String>
}

impl UserRegistryInput {

    pub async fn is_user_exists(&self, conn: &mut AsyncPgConnection) -> Result<Option<String>, DieselError> {
        let existed = select(exists(
            users.filter(crate::schema::users::columns::e_mail.eq(&self.email).or(crate::schema::users::columns::username.eq(&self.username)))
        ))
            .get_result::<bool>(conn)
            .await?;

        if existed {
            if select(exists(users.filter(crate::schema::users::columns::e_mail.eq(&self.email)))).get_result::<bool>(conn).await? {
                return Ok(Some("email".to_string()));
            }
            return Ok(Some("username".to_string()));
        }

        Ok(None)
    }

    pub fn is_email_valid(&self) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(&self.email)
    }
    
    pub fn is_username_valid(&self) -> bool {
        if self.username.len() < 3 {
            return false;
        }
        self.username.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
    
    pub fn is_password_valid(&self) -> bool {
        let length_regex = Regex::new(r"^.{8,}$").expect("Invalid regex");
        let uppercase_regex = Regex::new(r"[A-Z]").expect("Invalid regex");
        let digit_regex = Regex::new(r"\d").expect("Invalid regex");
        let special_regex = Regex::new(r"[?&%$#@!_]").expect("Invalid regex");

        if !length_regex.is_match(&self.password) { return false; }
        if !uppercase_regex.is_match(&self.password) { return false; }
        if !digit_regex.is_match(&self.password) { return false }
        if !special_regex.is_match(&self.password) { return false; }

        true
    }
    
    pub fn hashing_password(&self, argon2: Argon2) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(self.password.as_bytes(), &salt)?;

        Ok(password_hash.to_string())
    }

    pub async fn create_user(&self, hashed_password: &str, conn: &mut AsyncPgConnection) -> Result<User, CustomError> {

        let new_user = NewUser {
            first_name: &self.first_name,
            second_name: &self.second_name,
            username: &self.username,
            email: &self.email,
            password: &hashed_password,
            role_type: self.role_type.as_deref()
        };

        let result = insert_into(users)
            .values(&new_user)
            .get_result(conn)
            .await?;

        Ok(result)

    }
}
