use crate::db::users;
use chrono::{
    NaiveDateTime,
    offset::Local,
};
use juniper::GraphQLInputObject;

pub struct User {
    pub id: i32,
    pub name: String,
    pub profile: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<users::User> for User {
    fn from(user: users::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            profile: user.profile.unwrap_or_else(|| String::from("")),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub profile: Option<String>,
}

impl From<NewUser> for users::UserNewForm {
    fn from(new_user: NewUser) -> Self {
        Self {
            name: new_user.name,
            profile: new_user.profile,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub profile: Option<String>,
}

impl From<UpdateUser> for users::UserUpdateForm {
    fn from(update_user: UpdateUser) -> Self {
        Self {
            name: update_user.name,
            profile: update_user.profile,
            updated_at: Local::now().naive_local(),
        }
    }
}
