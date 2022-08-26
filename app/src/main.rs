mod config;
mod error;
mod models;

use axum::extract::Extension;
use axum::{response::Html, routing::get, Router};
use sqlx::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let db_pool = PgPool::connect(&config.database_url)
                                                            .await
                                                            .expect("Problem connecting to the database");

    let app = Router::new()
                        .route("/", get(handler))
                        .merge(statics::asset_pipeline_routes())
                        .merge(statics::image_routes())
                        .layer(Extension(db_pool))
                        .layer(Extension(config));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Extension(pool): Extension<PgPool>) -> Result<Html<String>, error::CustomError> {
    let users = models::user::User::get_users(&pool, 10).await?;

    let html = format!("<h1>Hello, World! We Have {} Users</h1>", users.len());

    
    Ok(layout::layout("Test", &html))
}

// Error here disabled with "rust-analyzer.diagnostics.disabled": ["macro-error"]
// in .vscode/settings.json
pub mod statics {
    include!(concat!(env!("OUT_DIR"), "/statics.rs"));
}