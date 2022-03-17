use crate::db::{
    PgPool,
    users::{
        User,
        UserNewForm,
        UserUpdateForm,
    },
    schema::users::dsl::*,
};
use actix_web::web::Data;
use anyhow::Result;
use diesel::{
    debug_query,
    dsl::{
        delete,
        insert_into,
        update,
    },
    pg::Pg,
    prelude::*,
};
use log::debug;

pub struct Repository;

impl Repository {
    // 全てのUserを配列として返す.
    pub fn all(pool: &Data<PgPool>) -> Result<Vec<User>> {
        let connection = pool.get()?;

        Ok(users.load(&connection)?)
    }

    // primary keyの配列から、これに合致するUserを配列として返す.
    pub fn any(pool: &Data<PgPool>, keys: &[i32]) -> Result<Vec<User>> {
        let connection = pool.get()?;
        let query = users.filter(id.eq_any(keys));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    // key_idに合致するUserを返す.
    pub fn find_by_id(pool: &Data<PgPool>, key_id: i32) -> Result<User> {
        let connection = pool.get()?;
        let query = users.find(key_id);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    // key_nameに合致するUserを配列として返す.
    pub fn find_by_name(pool: &Data<PgPool>, key_name: String) -> Result<Vec<User>> {
        let connection = pool.get()?;
        let query = users.filter(name.eq(key_name));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    // new_formを新しい行としてDBに追加し、その行のUserを返す.
    pub fn insert(pool: &Data<PgPool>, new_form: UserNewForm) -> Result<User> {
        let connection = pool.get()?;
        let query = insert_into(users).values(new_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    // key_idに合致するUserの行をupdate_formで更新し、その行のUserを返す.
    pub fn update(pool: &Data<PgPool>, key_id: i32, update_form: UserUpdateForm) -> Result<User> {
        let connection = pool.get()?;
        let query = update(users.find(key_id)).set(update_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    // idに合致するUserの行をDBから削除し、その行のUserを返す.
    pub fn delete(pool: &Data<PgPool>, key_id: i32) -> Result<User> {
        let connection = pool.get()?;
        let query = delete(users.find(key_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }
}
