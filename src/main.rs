use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};

mod apis;
mod models;
mod repositories;


#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
      let alert_db = match repositories::repository::Database::new().await {
        Ok(db) => db,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err.to_string())),
    };
    let app_data = web::Data::new(alert_db);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(apis::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}