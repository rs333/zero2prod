use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Parameters {
    #[allow(dead_code)]
    subscription_token: String,
}

#[tracing::instrument(
    name = "Confirm a pending subscriber",
    skip(parameters, pool,)
)]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let id = match get_subscriber_id_from_token(
        &pool,
        &parameters.subscription_token,
    )
    .await
    {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };

    match id {
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

pub async fn confirm_subscriber(
    pool: &PgPool,
    subscriber_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"update subscriptions set status = 'confirmed' where id = $1;"#,
    )
    .bind(subscriber_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "select subscriber_id, subscription_token from subscription_tokens \
        where subscription_token = $1",
        subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}
