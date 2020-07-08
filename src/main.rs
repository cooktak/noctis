#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::sync::Arc;
use std::io::{Error as StdError, ErrorKind as StdErrorKind};

use actix_web::{App, dev, Error, http, HttpResponse, HttpServer, middleware, Responder, web};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use juniper::http::{GraphQLRequest, playground::playground_source};

use crate::database::connection::{build_pool, establish_diesel_connection, establish_r2d2_connection};
use crate::database::error::DatabaseError;
use crate::gql::{context::Context, create_schema, Schema};
use crate::config::ConfigError;

mod config;
mod database;
mod device;
mod gql;
mod user;

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

embed_migrations!();

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::Config::load().map_err(|e| match e {
        ConfigError::MissingKey(_) => StdError::new(StdErrorKind::NotFound, e.to_string()),
        ConfigError::InvalidFormat(_) => StdError::new(StdErrorKind::InvalidInput, e.to_string()),
    })?;

    // Establish connection for diesel
    let connection = establish_diesel_connection(&config.database);

    // Migrate
    embedded_migrations::run(&connection.ok().expect("Database Error"))
    .map_err(|e| DatabaseError::MigrationError(e.to_string()))
    .unwrap();

    // Establish connection for R2D2
    let manager = establish_r2d2_connection(&config.database);

    // Build Connection pool
    let pool = build_pool(manager).map_err(|e| match e {
        DatabaseError::ConnectionError(_) => StdError::new(StdErrorKind::NotConnected, e.to_string()),
        DatabaseError::MigrationError(_) => StdError::new(StdErrorKind::ConnectionRefused, e.to_string()),
        DatabaseError::PoolError(_) => StdError::new(StdErrorKind::ConnectionRefused, e.to_string()),
    })?;

    // Create Context
    let context = Arc::new(Context::new(pool, config.clone()));

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