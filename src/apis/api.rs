use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::alert::Alert, repositories::repository::Database};


#[post("/alerts")]
pub async fn create_alert(db: web::Data<Database>, new_alert: web::Json<Alert>) -> HttpResponse {
    let alert = db.create_alert(new_alert.into_inner());
    match alert.await {
        Ok(alert) => HttpResponse::Ok().json(alert),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/alerts/{id}")]
pub async fn get_alert_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let alert = db.get_alert_by_id(&id).await;
    match alert {
        Ok(Some(alert)) => HttpResponse::Ok().json(alert),
        Ok(None) => HttpResponse::NotFound().json("Alert not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

#[get("/alerts")]
pub async fn get_alerts(db: web::Data<Database>) -> HttpResponse {
    let result   = db.get_alerts().await;
    match result  {
        Ok(alerts) => {
            
            HttpResponse::Ok().json(alerts)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

#[delete("/alerts/{id}")]
pub async fn delete_alert_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let deleted_alert = db.delete_alert_by_id(&id).await;
    match deleted_alert {
        Ok(Some(_alert)) => HttpResponse::Ok().json("Success"),
        Ok(None) => HttpResponse::NotFound().json("Alert not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

#[put("/alerts/{id}")]
pub async fn update_alert_by_id(db: web::Data<Database>, id: web::Path<String>, updated_alert: web::Json<Alert>) -> HttpResponse {
    let alert = db.update_alert_by_id(&id, updated_alert.into_inner()).await;
    match alert {
        Ok(Some(alert)) => HttpResponse::Ok().json(alert),
        Ok(None) => HttpResponse::NotFound().json("Alert not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_alert)
            .service(get_alert_by_id)
            .service(get_alerts)
            .service(delete_alert_by_id)
            .service(update_alert_by_id)
    );
}