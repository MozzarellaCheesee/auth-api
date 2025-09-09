use diesel::prelude::*;

#[derive(Queryable, Insertable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::issued_jwt_tokens)]
#[diesel(primary_key(jti))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IssuedJwtToken {
    pub jti: String,
    pub user_id: i64,
    pub revoked: bool,
    pub device_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::issued_jwt_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewIssuedJwtToken<'a> {
    pub jti: &'a str,
    pub user_id: i64,
    pub device_id: &'a str,
}