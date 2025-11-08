use actix_files::Files;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::env;
use std::io;

#[derive(Clone)]
struct AppConfig {
    email: String,
    logo_path: String,
    logo_name: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let static_dir = "./public";
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let email = env::var("EMAIL").unwrap_or_else(|_| "info@domain.local".to_string());
    let logo_path =
        env::var("LOGO_PATH").unwrap_or_else(|_| "https://placehold.co/400x300".to_string());
    let logo_name = env::var("LOGO_NAME").unwrap_or_else(|_| "logo.png".to_string());

    log::info!("Serving {} on HTTP port: {}", static_dir, port);
    log::info!("Contact email: {}", email);
    log::info!("Logo path: {}", logo_path);
    log::info!("Logo name: {}", logo_name);

    let config = web::Data::new(AppConfig {
        email,
        logo_path,
        logo_name,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Endpoint to get contact email
            .route("/config", web::get().to(get_config))
            // Serve static files from the public directory
            .service(Files::new("/", static_dir).index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn get_config(config: web::Data<AppConfig>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "email": config.email,
        "logo_path": config.logo_path,
        "logo_name": config.logo_name,
    }))
}
