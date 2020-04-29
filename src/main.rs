use diesel::pg::PgConnection;
use diesel::prelude::*;
use get_started_diesel_insert::{schema, NewUser, User};

const DATABASE_URL: &'static str = "postgres://postgres:@localhost:5432";

fn main() {
  let connection =
    PgConnection::establish(DATABASE_URL).expect(&format!("Error connecting to {}", DATABASE_URL));

  let baz_user = NewUser { name: "baz".into() };
  let qux_user = NewUser { name: "qux".into() };

  let users = diesel::insert_into(schema::users::table)
    .values(vec![baz_user, qux_user])
    .get_results::<User>(&connection)
    .expect("Error saving new users");
  println!("{:#?}", users);
}
