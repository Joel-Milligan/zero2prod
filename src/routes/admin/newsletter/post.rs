use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::PgPool;

use crate::authentication::UserId;
use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::idempotency::{save_response, try_processing, IdempotencyKey, NextAction};
use crate::utils::{e400, e500, see_other};

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    html: String,
    text: String,
    idempotency_key: String,
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(form, pool, email_client),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn publish_newsletter(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    user_id: ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let FormData {
        title,
        text,
        html,
        idempotency_key,
    } = form.0;

    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;

    let transaction = match try_processing(&pool, &idempotency_key, *user_id)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            success_message().send();
            return Ok(saved_response);
        }
    };

    let subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;
    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(&subscriber.email, &title, &html, &text)
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {}", subscriber.email)
                    })
                    .map_err(e500)?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    error.message = %error,
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid",
                );
            }
        }
    }
    success_message().send();
    let response = see_other("/admin/newsletter");
    let response = save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(e500)?;
    Ok(response)
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();

    Ok(confirmed_subscribers)
}

fn success_message() -> FlashMessage {
    FlashMessage::info("The newsletter issue has been published!")
}
