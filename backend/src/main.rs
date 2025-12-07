mod db;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use std::env;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "Squad Planner API"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Get configuration from environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./squad_planner.db".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    log::info!("Starting Squad Planner API server");
    log::info!("Host: {}", host);
    log::info!("Port: {}", port);

    // Initialize database
    let pool = db::initialize(&database_url)
        .await
        .expect("Failed to initialize database");

    log::info!("Database initialized successfully");

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/api/health", web::get().to(health_check))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
