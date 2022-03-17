use crate::db::schema::users;
use chrono::NaiveDateTime;

mod repository;
pub use repository::Repository;

// Identifiable: この構造体がDBのテーブルであることを示す.
// Queryable: この構造体がDBに問い合わせることができることを示す.
// Clone: おまけ.
#[derive(Clone, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub profile: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Insertable: この構造体がDBに新しい行を挿入できることを示す.
#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNewForm {
    pub name: String,
    pub profile: Option<String>,
}

// AsChangeset: この構造体がDBの任意の行に変更を加えられることを示す.
#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdateForm {
    pub name: Option<String>,
    pub profile: Option<String>,
    pub updated_at: NaiveDateTime,
}
