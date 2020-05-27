use std::sync::Arc;

use actix_web::{App, dev, Error, http, HttpResponse, HttpServer, middleware, Responder, web};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use juniper::http::{GraphQLRequest, playground::playground_source};

use database::connection::establish_connection;
use gql::{create_schema, Schema};

use crate::gql::context::Context;

mod gql;
mod config;
mod database;

async fn playground() -> impl Responder {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

async fn graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<Arc<Context>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(user))
}

fn render_404<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    dbg!(res.request());
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::Config::load().expect("Config Error");

    let pool = establish_connection(&config.database).expect("Invalid database config");

    // Create Context
    let context = Arc::new(Context::new(pool));

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    let server = HttpServer::new(move || {
        App::new()
        .data(schema.clone())
        .data(context.clone())
        .wrap(middleware::Logger::default())
        .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, render_404))
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphql").route(web::get().to(graphql)))
        .service(web::resource("/").route(web::get().to(playground)))
    }).keep_alive(75);

    server.bind(&config.bind_address)?.run().await
}