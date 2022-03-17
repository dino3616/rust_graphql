use crate::{
    schemas::{
        root::{
            Context,
            Mutation,
            Query,
        },
        user::User,
    },
};
use juniper::{
    graphql_object,
};

// 「GraphQLのオブジェクト型」という特徴を付与する.
#[graphql_object(context=Context)]
impl Query {
    // 今回は導入編なので、リゾルバも簡易的な感じで.
    fn dummy_query() -> User {
        use chrono::offset::Local;

        // ダミーのUserオブジェクトを返す.
        User {
            id: 0,
            name: "yukarisan-lover".to_string(),
            profile: "I love yukari-san forever...!".to_string(),
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

#[graphql_object(context=Context)]
impl Mutation {
    fn dummy_mutation() -> String {
        String::from("It is dummy mutation.")
    }
}
