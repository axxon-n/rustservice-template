use actix_cors::Cors;
use tokio::sync::Semaphore;
use std::sync::Arc;
use actix_web::{
    http,
    middleware, 
    web, 
    App, 
    HttpServer
};
use zlogisticapps::app_config::{Config};
use zlogisticapps::common::{AppState};
use tracing::{info};

use zlogisticapps::service::{
    health_check
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::from_env().expect("Server configuration");

    println!("{:?}", config);

    let port = config.port;
    let host = config.host.clone();
    let num_cpus = num_cpus::get();
    let parallel_files = config.parallel_files;
    let payload_max_size = config.payload_max_size;

    info!(
        "Starting application. Num CPUs {}. Max Parallel Files {}",
        num_cpus, parallel_files
    );

    let sem = Arc::new(Semaphore::new(parallel_files));
    let data = web::Data::new(AppState {
        //db_svc: db,
        semaphore: sem,
        is_test: config.is_test == 1
    });

    info!("Starting server at http://{}:{}/", host, port);

    HttpServer::new(move || {

        let cors = Cors::default()
          .allow_any_origin()
          .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
          .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::HeaderName::from_static("ngrok-skip-browser-warning")])
          .allowed_header(http::header::CONTENT_TYPE)
          .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::PayloadConfig::new(payload_max_size))
            .app_data(data.clone())
            .route("/", web::get().to(health_check::index))
            // .route("/v2/buildhub-event", web::post().to(buildhub_event::index))
            // .route("/v2/buildhub-event", web::get().to(buildhub_event::index))
            // .route("/v2/buildhub-event", web::put().to(buildhub_event::index))
            // .route("/v2/buildhub-event", web::delete().to(buildhub_event::index))
    })
    .bind(format!("{}:{}", host, port))?
    .workers(num_cpus * 2)
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_zero() {
        assert_eq!(3, 3);
    }

    #[tokio::test]
    async fn test_one() {
        assert_eq!(3, 3);
    }
}
