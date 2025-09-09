use diesel::prelude::*;

#[derive(Queryable, Insertable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub second_name: String,
    pub username: String,
    #[diesel(column_name = e_mail)]
    pub email: String,
    pub password: String,
    #[diesel(column_name = e_mail_confirmed)]
    pub email_confirmed: bool,
    pub role_type: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub second_name: &'a str,
    pub username: &'a str,
    #[diesel(column_name = e_mail)]
    pub email: &'a str,
    pub password: &'a str,
    pub role_type: Option<&'a str>
}