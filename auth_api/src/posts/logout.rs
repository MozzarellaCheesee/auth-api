use actix_web::{post, web, HttpResponse, Responder};
use argon2::Argon2;
use crate::ConnPool;
use crate::error::CustomError;
use crate::structs::{AuthLogoutInput, Claims};

#[post("/api/auth/logout")]
pub async fn logout(
    token: web::Json<AuthLogoutInput>,
    _argon2: web::Data<Argon2<'_>>,
    pool: web::Data<ConnPool>,
) -> impl Responder {
    let mut conn = pool.get().await.expect("Не удалось получить соединение");
    println!("Начало выхода");

    let data: Claims = match token.transcript_token() {
        Ok(data) => data,
        Err(err) => return match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Err(CustomError::ExpiredTokenError("Токен просрочен".to_string()))
            }
            _ => Err(CustomError::TokenCreationError(err))
        }
    };

    println!("дата получена");

    match token.set_revoked(&mut conn, &data.jti).await {
        Ok(result) => {
            println!("запись в бд");
            if result.revoked {
                Ok(HttpResponse::Ok().json(&token))
            } else {
                Err(CustomError::RevorkTokenError("Токен не был отзован".to_string()))
            }
        }
        Err(err) => Err(CustomError::DbError(err))
    }

}