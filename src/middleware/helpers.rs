use crate::utils::generate_api_key_prefix;
use crate::{
    entities::{api_keys, prelude::*},
    middleware::error::AuthError,
    state::AppState,
};
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use bcrypt::verify;
use chrono::Utc;
use sea_orm::*;
use tokio::spawn;
use uuid::Uuid;

pub(crate) async fn find_and_validate_key(
    api_key: &str,
    organization_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<api_keys::Model, AuthError> {
    let key_prefix = generate_api_key_prefix(api_key);

    let potential_keys = api_keys::Entity::find()
        .filter(api_keys::Column::KeyPrefix.eq(key_prefix))
        .filter(api_keys::Column::OrganizationId.eq(*organization_id))
        .all(db)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

    let key = potential_keys
        .into_iter()
        .find(|k| verify(api_key, &k.key).unwrap_or(false))
        .ok_or_else(|| AuthError::InvalidToken("Invalid API key".into()))?;

    // Checks expiration first
    if let Some(expires_at) = key.expires_at {
        if expires_at < Utc::now().naive_utc() {
            return Err(AuthError::ExpiredToken);
        }
    }

    // We clone what we need for the background task
    let db = db.clone();
    let key_id = key.id;

    // Update last_used_at in the background so that we don't block the response
    spawn(async move {
        let now = Utc::now().naive_utc();

        let key_active: api_keys::ActiveModel = api_keys::ActiveModel {
            id: Set(key_id),
            last_used_at: Set(Some(now)),
            updated_at: Set(now),
            ..Default::default()
        };

        if let Err(e) = key_active.update(&db).await {
            tracing::error!("Failed to update key last_used_at: {}", e);
        }
    });

    Ok(key)
}

pub(crate) async fn track_api_usage(state: &AppState, org_id: &Uuid) -> Result<(), AuthError> {
    let org = match Organizations::find_by_id(*org_id)
        .one(&state.db.connection)
        .await
    {
        Ok(Some(org)) => org,
        Ok(None) => {
            tracing::error!("Organization not found for usage tracking: {}", org_id);
            return Ok(()); // Don't fail the request for tracking errors
        }
        Err(e) => {
            tracing::error!("Database error in usage tracking: {}", e);
            return Ok(()); // Don't fail the request for tracking errors
        }
    };

    // If no stripe customer id, early return success
    let Some(stripe_subscription_id) = org.stripe_subscription_id else {
        return Ok(());
    };

    tracing::debug!(
        "Reporting usage to Stripe for stripe_customer_id {}",
        stripe_subscription_id
    );

    // Report usage to Stripe
    if let Err(e) = state.stripe.report_api_usage(&stripe_subscription_id).await {
        tracing::error!("Failed to report usage to Stripe: {}", e);
    }

    Ok(())
}

// Helper function to extract API key from headers
pub(crate) fn extract_api_key(parts: &Parts) -> Result<String, AuthError> {
    parts
        .headers
        .get("X-API-Key")
        .ok_or(AuthError::MissingToken)?
        .to_str()
        .map(String::from)
        .map_err(|_| AuthError::InvalidToken("Invalid header value".into()))
}

pub(crate) async fn extract_organization_id(
    parts: &mut Parts,
    state: &AppState,
) -> Result<Uuid, AuthError> {
    // Try standard organization path first
    if let Ok(Path(org_path)) = Path::<(Uuid,)>::from_request_parts(parts, state).await {
        return Ok(org_path.0);
    }

    // Try nested organization path next
    if let Ok(Path(nested_path)) = Path::<(Uuid, Uuid)>::from_request_parts(parts, state).await {
        return Ok(nested_path.0);
    }

    Err(AuthError::OrgNotFound)
}
