use actix_web::{
    Error,
    HttpResponse,
    web::{
        Data,
        Payload,
    },
};
use juniper_actix::{
    graphiql_handler,
    graphql_handler,
    playground_handler,
};

pub mod resolvers;
pub mod schemas;
use crate::schemas::root::{
    Context,
    Schema,
};

// Actix WebからGraphQLにアクセスするためのハンドラメソッド.
pub async fn graphql(req: actix_web::HttpRequest, payload: Payload, schema: Data<Schema>) -> Result<HttpResponse, Error> {
    // tokenがリクエストヘッダに添付されている場合はSomeを、なければNoneを格納する.
    let token = req
        .headers()
        .get("token")
        .map(|t| t.to_str().unwrap().to_string());

    let context = Context {
        token,
    };

    graphql_handler(&schema, &context, req, payload).await
}

// Actix WebからGraphiQLにアクセスするためのハンドラメソッド.
pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

// Actix WebからGraphQL Playgroundにアクセスするためのハンドラメソッド.
pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
