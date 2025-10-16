use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use argon2::Argon2;
use crate::ConnPool;
use crate::error::CustomError;
use crate::structs::{AuthLogoutInput, Claims};

#[post("/api/auth/logout")]
pub async fn logout(
    mut token: web::Json<AuthLogoutInput>,
    _argon2: web::Data<Argon2<'_>>,
    pool: web::Data<ConnPool>,
) -> impl Responder {
    let mut conn = pool.get().await.expect("Не удалось получить соединение");

    let check: String = token.refresh_token.chars().take(7).collect();

    if (check != "Bearer ") {
        return CustomError::TokenIsNotValid("Неправильно указан токен".to_string()).error_response()
    }

    let byte_offset: usize = token.refresh_token.chars().take(7).map(|c| c.len_utf8()).sum();
    token.refresh_token.drain(0..0);

    let data: Claims = match token.transcript_token() {
        Ok(data) => data,
        Err(err) => return match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                CustomError::ExpiredTokenError("Токен просрочен".to_string()).error_response()
            }
            _ => CustomError::TokenCreationError(err).error_response(),
        }
    };

    match token.set_revoked(&mut conn, &data.jti).await {
        Ok(result) => {
            println!("запись в бд");
            if result.revoked {
                HttpResponse::Ok().json(&token).into()
            } else {
                CustomError::RevorkTokenError("Токен не был отзован".to_string()).error_response()
            }
        }
        Err(err) => CustomError::DbError(err).error_response(),
    }
}