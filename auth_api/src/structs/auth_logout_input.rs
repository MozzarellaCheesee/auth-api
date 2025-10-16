use diesel::{ExpressionMethods};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::structs::Claims;
use diesel::{update, QueryDsl};
use crate::schema::issued_jwt_tokens::dsl::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::models::IssuedJwtToken;
use crate::schema::issued_jwt_tokens::revoked;

#[derive(Deserialize, Serialize)]
pub struct AuthLogoutInput {
    pub refresh_token: String,
    pub device_id: String,
}

impl AuthLogoutInput {

    pub async fn set_revoked(&self, conn: &mut AsyncPgConnection, token_jti: &str) -> Result<IssuedJwtToken, diesel::result::Error> {

        update(issued_jwt_tokens.find(token_jti))
            .set(revoked.eq(true))
            .get_result(conn)
            .await
    }

    pub fn transcript_token(&self) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET должен быть установлен");

        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let token_data = decode::<Claims>(&self.refresh_token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}