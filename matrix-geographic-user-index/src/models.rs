use diesel::pg::types::sql_types::Uuid;
use diesel::{Queryable, Insertable};
use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub lobsters_address: String,
    pub matrix_nick: String,
    pub healpix_region: i64,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub lobsters_address: &'a str,
    pub matrix_nick: &'a str,
    pub healpix_region: i64,
}
