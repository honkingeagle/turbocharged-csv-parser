use csv_upload_server::AppState;
use sqlx::mysql::MySqlPool;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let database_url = env::var("DB_URL").unwrap();
    let frontend_url = env::var("FRONTEND_URL").unwrap();

    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let state = Arc::new(AppState::new(pool, frontend_url));

    let app = csv_upload_server::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
