use actix_cors::Cors;
use actix_web::{
    App,
    http::header,
    HttpServer,
    middleware::{
        Compress,
        Logger,
    },
    web::{
        self,
        Data,
    },
};
use anyhow::Result;
use dotenv::dotenv;
use graphql::{
    db::new_pool,
    graphiql,
    graphql,
    playground,
    schemas::create_schema,
};
use std::{
    env,
    sync::Arc,
};

// 今回サーバーの実装にActix Webを使用しているので、非同期ランタイムはactix-rtを採用.
#[actix_rt::main]
async fn main() -> Result<()> {
    // .envに記述された環境変数の読み込み.
    dotenv().ok();

    // debugと同等以上の重要度を持つログを表示するように設定し、ログを開始する.
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Schemaオブジェクトをスレッドセーフな型でホランラップする.
    let schema = Arc::new(create_schema());
    // PgPoolオブジェクトをスレッドセーフな型でホランラップする.
    let pool = Arc::new(new_pool()?);

    // サーバーの色んな設定.
    let mut server = HttpServer::new(move || {
        App::new()
            // SchemaオブジェクトをActix Webのハンドラメソッドの引数として使えるようにする.
            .app_data(Data::from(schema.clone()))
            // PgPoolオブジェクトをActix Webのハンドラメソッドの引数として使えるようにする.
            .app_data(Data::from(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Compress::default())
            .wrap(Logger::default())
            // /graphqlエンドポイントにgraphql()をセットする.
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            // /graphiqlエンドポイントにgraphiql()をセットする.
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            // /playgroundエンドポイントにplayground()をセットする.
            .service(web::resource("/playground").route(web::get().to(playground)))
    });

    // Herokuとかにデプロイすることを考えて、HOSTやPORTの環境変数を優先する.
    let host = match env::var("HOST") {
        Ok(ok) => ok,
        Err(_) => env::var("LOCAL_HOST")?,
    };
    let port = match env::var("PORT") {
        Ok(ok) => ok,
        Err(_) => env::var("LOCAL_PORT")?,
    };
    let address = format!("{}:{}", host, port);
    server = server.bind(address)?;
    server.run().await?;

    Ok(())
}
