use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subsciber.",
        %request_id,
        subsciber_email = %form.email,
        subsciber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    tracing::info!(
        "[{request_id}] Adding '{}' '{}' as a new subscriber.",
        form.name,
        form.email
    );
    tracing::info!("[{request_id}] Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("[{request_id}] New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("[{request_id}] Failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
