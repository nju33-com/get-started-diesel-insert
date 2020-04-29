#[macro_use]
extern crate diesel;

pub mod schema;

use schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub name: String,
}

#[derive(Queryable, Debug)]
pub struct User {
  pub id: i32,
  pub name: String,
}
