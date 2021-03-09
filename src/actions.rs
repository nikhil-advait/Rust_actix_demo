use chrono::{NaiveDateTime, NaiveDate};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

/// Run query using Diesel to find user by uid and return it.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(user_id.eq(uid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    first_n: &str,
    last_n: &str,
    email_str: &str,
    passwd: &str,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        user_id: Uuid::new_v4(),
        first_name: first_n.to_owned(),
        last_name: last_n.to_owned(),
        email: email_str.to_owned(),
        password: passwd.to_owned(),
        created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}
