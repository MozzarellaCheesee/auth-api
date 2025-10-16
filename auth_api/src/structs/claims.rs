use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use chrono::{Utc, Duration};
use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::error::CustomError;
use crate::models::{IssuedJwtToken, NewIssuedJwtToken, User};
use crate::schema::issued_jwt_tokens::dsl::issued_jwt_tokens;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    exp: i64,
    iat: i64,
    pub jti: String,
    #[serde(rename = "type")]
    token_type: String,
    device_id: String,
    username: String,
    role: String,
}

impl Claims {

    fn generate_jti() -> String { uuid::Uuid::new_v4().to_string() }

    pub fn new(
        token_type: &str,
        duration: Duration,
        device_id: &str,
        user: &User
    ) -> Self {
        let current_timestamp = Utc::now().timestamp();
        let exp_timestamp = (Utc::now() + duration).timestamp();

        let role_value: String = match &user.role_type {
            Some(role) => role.clone(),
            None => "guest".to_string(),
        };

        Self {
            iss: "auth_service".to_string(),
            sub: user.id.to_string(),
            exp: exp_timestamp,
            iat: current_timestamp,
            jti: Self::generate_jti(),
            token_type: token_type.to_string(),
            device_id: device_id.to_string(),
            username: user.username.to_string(),
            role: role_value
        }
    }

    pub fn generate_token(
        &self
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET не установлен");
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let header = Header::new(Algorithm::HS256);
        encode(&header, &self, &encoding_key)
    }

    pub async fn save_token(&self, conn: &mut AsyncPgConnection, found_user: &User) -> Result<IssuedJwtToken, CustomError> {

        let new_auth = NewIssuedJwtToken {
            jti: &self.jti,
            user_id: found_user.id,
            device_id: &self.device_id.to_string(),
        };

        let result = insert_into(issued_jwt_tokens)
            .values(&new_auth)
            .get_result::<IssuedJwtToken>(conn)
            .await?;

        Ok(result)
    }

}