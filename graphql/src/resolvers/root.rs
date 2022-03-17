use crate::{
    db::users,
    schemas::{
        root::{
            Context,
            Mutation,
            Query,
        },
        user::{
            User,
            NewUser,
            UpdateUser,
        },
    },
};
use juniper::{
    FieldResult,
    graphql_object,
};

// 「GraphQLのオブジェクト型」という特徴を付与する.
#[graphql_object(context=Context)]
impl Query {
    fn get_user(context: &Context, id: i32) -> FieldResult<User> {
        let user = users::Repository::find_by_id(&context.pool, id)?;

        Ok(user.into())
    }

    #[graphql(
        arguments(
            start(default = 0),
            range(default = 50),
        )
    )]
    async fn list_user(context: &Context, name: String, start: i32, range: i32) -> FieldResult<Vec<User>> {
        // asよりも安全に型変換を行う.
        let start: usize = start.try_into()?;
        let range: usize = range.try_into()?;
        let end = start + range;

        let users = users::Repository::find_by_name(&context.pool, name)?;

        // 引数に合わせてベクタをスライスする.
        let users = match users.len() {
            n if n > end => users[start..end].to_vec(),
            n if n > start => users[start..].to_vec(),
            _ => Vec::new(),
        };

        Ok(users.into_iter().map(|u| u.into()).collect())
    }
}

#[graphql_object(context=Context)]
impl Mutation {
    fn create_user(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let user = users::Repository::insert(&context.pool, new_user.into())?;

        Ok(user.into())
    }

    fn update_user(context: &Context, id: i32, update_user: UpdateUser) -> FieldResult<User> {
        let user = users::Repository::update(&context.pool, id, update_user.into())?;

        Ok(user.into())
    }

    fn delete_user(context: &Context, id: i32) -> FieldResult<User> {
        let user = users::Repository::delete(&context.pool, id)?;

        Ok(user.into())
    }
}
