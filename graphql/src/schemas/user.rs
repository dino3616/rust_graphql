use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;
// use crate::db::{
//     users,
// };

pub struct User {
    pub id: i32,
    pub name: String,
    pub profile: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// impl From<users::User> for User{
//     fn from(user: users::User)->Self{
//         Self{
//             id: user.id,
//             name: user.name,
//             profile: user.password,
//             created_at: user.created_at,
//             updated_at: user.updated_at,
//         }
//     }
// }

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub profile: Option<String>,
}

// impl From<NewUser> for users::UserNewForm{
//     fn from(new_user: NewUser)->Self{
//         Self{
//             id: new_user.id,
//             name: new_user.name,
//             password: new_user.password,
//             token: new_user.token,
//         }
//     }
// }

#[derive(GraphQLInputObject)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub profile: Option<String>,
}

// impl From<UpdateUser> for users::UserUpdateForm{
//     fn from(update_user: UpdateUser)->Self{
//         Self{
//             id: update_user.id,
//             name: update_user.name,
//             password: update_user.password,
//             token: update_user.token,
//         }
//     }
// }
