use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use argon2::{Argon2};
use diesel::result::Error;
use crate::ConnPool;
use crate::error::CustomError;
use crate::structs::UserRegistryInput;

#[post("/api/auth/registry")]
pub async fn registry(
    user_data: web::Json<UserRegistryInput>,
    argon2: web::Data<Argon2<'_>>,
    pool: web::Data<ConnPool>,
) -> impl Responder {
    let mut conn = pool.get().await.expect("Не удалось получить соединение");

    match user_data.is_user_exists(&mut conn).await {
        Ok(Some(field)) if field == "email" => {
            return CustomError::EmailAlreadyExists("Такой e-mail уже зарегистрирован".to_string()).error_response();
        },
        Ok(Some(field)) if field == "username" => {
            return CustomError::UsernameAlreadyExists("Такой username уже зарегистрирован".to_string()).error_response();
        }
        _ => {}
    }

    if !user_data.is_username_valid() {
        return CustomError::InvalidInput(
            "Юзернейм может содержать только буквы и знак `_`".to_string(),
        ).error_response();
    }

    if !user_data.is_email_valid() {
        return CustomError::InvalidInput(
            "Некорректный e-mail".to_string(),
        ).error_response();
    }

    if !user_data.is_password_valid() {
        return CustomError::InvalidInput(
            "Пароль должен содержать не менее 8 символов, одну или более заглавную букву и специальный символ".to_string(),
        ).error_response();
    }


    let hashed_password = match user_data.hashing_password(argon2.get_ref().clone()) {
        Ok(hashed_password) => hashed_password,
        Err(err) => return CustomError::HashingError(err).error_response()
    };

    match user_data.create_user(&hashed_password, &mut conn).await {
        Ok(user) => { HttpResponse::Ok().json(user.email) },
        Err(err) => err.error_response()
    }

}

