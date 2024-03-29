use diesel::data_types::PgTimestamp;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub original: String,
    pub created: PgTimestamp,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::links)]
pub struct NewLink<'a> {
    pub short: &'a String,
    pub original: &'a String,
}
