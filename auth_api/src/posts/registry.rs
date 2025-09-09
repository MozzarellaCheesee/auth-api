use actix_web::{post, web, HttpResponse, Responder};
use argon2::{Argon2};
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

    match user_data.is_user_exists(&mut conn).await? {
        Some(field) if field == "email" => {
            return Err(CustomError::EmailAlreadyExists("Такой e-mail уже зарегистрирован".to_string()))
        },
        Some(field) if field == "username" => {
            return Err(CustomError::UsernameAlreadyExists("Такой username уже зарегистрирован".to_string()))
        },
        Some(_) => unreachable!(),
        None => (),
    }

    if !user_data.is_username_valid() {
        return Err(CustomError::InvalidInput(
            "Юзернейм может содержать только буквы и знак `_`".to_string(),
        ))
    }

    if !user_data.is_email_valid() {
        return Err(CustomError::InvalidInput(
            "Некорректный e-mail".to_string(),
        ))
    }

    if !user_data.is_password_valid() {
        return Err(CustomError::InvalidInput(
            "Пароль должен содержать не менее 8 символов, одну или более заглавную букву и специальный символ".to_string(),
        ))
    }


    let hashed_password = match user_data.hashing_password(argon2.get_ref().clone()) {
        Ok(hashed_password) => hashed_password,
        Err(err) => return Err(CustomError::HashingError(err)),
    };

    match user_data.create_user(&hashed_password, &mut conn).await {
        Ok(user) => { Ok(HttpResponse::Ok().json(user.email)) },
        Err(err) => Err(err)
    }

}

