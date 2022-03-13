use crate::{
    schemas::{
        root::{
            Mutation,
            Query,
        },
    },
};
use juniper::{
    graphql_object,
};

// 「GraphQLのオブジェクト型」という特徴を付与する.
#[graphql_object]
impl Query {
    // 今回は導入編なので、リゾルバも簡易的な感じで.
    fn dummy_query() -> String {
        String::from("It is dummy query.")
    }
}

#[graphql_object]
impl Mutation {
    fn dummy_mutation() -> String {
        String::from("It is dummy mutation.")
    }
}
