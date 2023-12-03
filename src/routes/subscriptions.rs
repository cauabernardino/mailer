use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %req_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _ = request_span.enter();
    // tracing::info!("Saving new subscriber to the database", req_id);

    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("ReqID: {} - New subscriber details have been saved", req_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("ReqID: {} - Failed to execute query: {:?}", req_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
