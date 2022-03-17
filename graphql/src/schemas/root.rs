use crate::db::{
    PgPool,
};
use actix_web::web::Data;
use juniper::{
    // 今回はSubscriptionを使わないので、ダミーの型を使う必要がある.
    EmptySubscription,
    RootNode,
};

// 後々ジェネリクスの引数とかに使うので、型をまとめておく.
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub struct Context {
    // 今回のシリーズではなんの括約もしないtokenﾆｷ.
    pub token: Option<String>,
    pub pool: Data<PgPool>,
}

// 「GraphQLのコンテキスト」という特徴を付与する.
impl juniper::Context for Context {}

pub struct Query;

pub struct Mutation;
