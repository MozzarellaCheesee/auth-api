# Authorization API Microservice

## О проекте
Микросервис для аутентификации пользователей в API. Он поддерживает базовые операции: регистрацию, вход и выход. Проект предназначен для интеграции в большие системы, где нужна простая и безопасная авторизация.

## Технологии
- Язык: Rust, C#.
- Фреймворки: actix_web, ocelot, AspNetCore.
- База данных: PostgreSQL.

## Установка
1. Клонируйте репозиторий:
   ```
   git clone https://github.com/MozzarellaCheesee/auth-api.git
   ```
2. Настройте окружение:
   создайте .env файл с переменными: DB_URL, SECRET_KEY.
3. Соберите docker контейнеры:
   ```bash
   docker build -t image-name .
   ```
4. Запустите docker-compose файл:
   ```bash
   docker-compose up -d --build
   ```

## Использование
Сервис работает через HTTP POST-запросы. Базовый URL: `http://localhost:5000` (настройте порт).

### Эндпоинты
- **Регистрация (/registry)**: POST-запрос с данными пользователя (логин, пароль). Возвращает токен.
  Пример:
  ```
  curl -X POST -H "Content-Type: application/json" -d '{"first_name": "first_name", "second_name": "second_name", "username": "username", "e-mail": "e-mail", "password": "password"}' http://localhost:5000/api/auth/registry
  ```
- **Вход (/login)**: POST-запрос для аутентификации. Возвращает JWT-токен.
  Пример:
  ```
  curl -X POST -H "Content-Type: application/json" -d '{"login": "login", "password": "password", "device_id": "device_id"}' http://localhost:5000/api/auth/login
  ```
- **Выход (/logout)**: POST-запрос для invalidate токена (требует авторизации).
  Пример:
  ```
  curl -X POST -H "Content-Type: application/json" -d '{"refresh_token": "refresh_token", "device_id": "device_id"}' http://localhost:5000/api/auth/logout
  ```

## Контрибьютинг
Форкните репозиторий, создайте ветку, внесите изменения и отправьте pull request.

## Лицензия
MIT License. Подробности в файле LICENSE.