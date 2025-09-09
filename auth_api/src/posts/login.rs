use actix_web::{post, web, HttpResponse, Responder};
use argon2::Argon2;
use chrono::Duration;
use crate::ConnPool;
use crate::error::CustomError;
use crate::structs::{AuthOutput, Claims, AuthInput};

#[post("/api/auth/login")]
pub async fn login(
    user_data: web::Json<AuthInput>,
    argon2: web::Data<Argon2<'_>>,
    pool: web::Data<ConnPool>,
) -> impl Responder {
    let mut conn = pool.get().await.expect("Не удалось получить соединение");

    let found_user = match user_data.find_user(&mut conn).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(CustomError::UserIsNotExist("Пользователь не зарегистрирован".to_string())),
        Err(err) => return Err(CustomError::DbError(err)),
    };

    let res = match user_data.verify_password(&found_user.password, argon2.get_ref().clone()) {
        Ok(result) => {
            result
        },
        Err(err) => {
            return Err(CustomError::HashingError(err));
        }
    };

    if !found_user.email_confirmed {
        return Err(CustomError::EmailNotConfirmed("Почта не подтверждена".to_string()))
    }

    user_data.is_authorized(&mut conn, &found_user).await.expect("Ошибка базы данных");

    if res {
        let access_token = Claims::new("access", Duration::hours(1), &user_data.device_id.to_string(),&found_user);
        let a_token = match access_token.generate_token() {
            Ok(token) => token,
            Err(err) => return Err(CustomError::TokenCreationError(err)),
        };

        let refresh_token = Claims::new("refresh", Duration::days(30), &user_data.device_id.to_string(), &found_user);
        let r_token = match refresh_token.generate_token() {
            Ok(token) => token,
            Err(err) => return Err(CustomError::TokenCreationError(err)),
        };
        match refresh_token.save_token(&mut conn, &found_user).await {
            Ok(data) => Ok(HttpResponse::Ok().json(AuthOutput::new(&a_token, &r_token, &data.device_id.to_string()))),
            Err(err) => Err(err)
        }

    } else {
        Err(CustomError::WrongPasswordError("Неправильный пароль".to_string()))
    }


}

