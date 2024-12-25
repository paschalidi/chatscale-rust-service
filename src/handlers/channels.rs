use crate::entities::channels;
use crate::state::AppState;
use crate::utils::ServerResponse;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::entities::prelude::Channels;
use crate::middleware::usage_tracking::ApiKeyAuth;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateChannelResponse {
    id: Uuid,
    name: String,
}

pub async fn create_channel(
    State(state): State<AppState>,
    auth: ApiKeyAuth,
    Json(payload): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let db = &state.db.connection;

    match Channels::find()
        .filter(channels::Column::Name.eq(&payload.name))
        .filter(channels::Column::OrganizationId.eq(auth.organization_id))
        .one(db)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => return ServerResponse::bad_request("Channel with this name already exists"),
        Err(err) => return ServerResponse::server_error(err, "Failed to check channel name"),
    }

    let new_channel = channels::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name.clone()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        organization_id: Set(auth.organization_id),
    };

    match Channels::insert(new_channel).exec(db).await {
        Ok(channel) => ServerResponse::ok(CreateChannelResponse {
            id: channel.last_insert_id,
            name: payload.name,
        }),
        Err(err) => ServerResponse::server_error(err, "Failed to create channel"),
    }
}

pub async fn get_channel_by_id(
    state: State<AppState>,
    auth: ApiKeyAuth,
    Path(channel_id): Path<String>,
) -> impl IntoResponse {
    let db = &state.db.connection;

    let channel_id = match Uuid::parse_str(&channel_id) {
        Ok(id) => id,
        Err(_) => {
            return ServerResponse::bad_request("Invalid channel ID");
        }
    };

    match Channels::find_by_id(channel_id)
        .filter(channels::Column::OrganizationId.eq(auth.organization_id))
        .one(db)
        .await
    {
        Ok(Some(channel)) => {
            let response = CreateChannelResponse {
                id: channel.id,
                name: channel.name,
            };
            ServerResponse::ok(response)
        }
        Ok(None) => ServerResponse::not_found("Channel not found"),
        Err(err) => ServerResponse::server_error(err, "Failed to fetch channel"),
    }
}

pub async fn get_channels(State(state): State<AppState>, auth: ApiKeyAuth) -> impl IntoResponse {
    match Channels::find()
        .filter(channels::Column::OrganizationId.eq(auth.organization_id))
        .order_by_desc(channels::Column::CreatedAt)
        .all(&state.db.connection)
        .await
    {
        Ok(channels) => ServerResponse::ok(
            channels
                .into_iter()
                .map(|channel| CreateChannelResponse {
                    id: channel.id,
                    name: channel.name,
                })
                .collect::<Vec<_>>(),
        ),
        Err(err) => ServerResponse::server_error(err, "Failed to fetch channels"),
    }
}
