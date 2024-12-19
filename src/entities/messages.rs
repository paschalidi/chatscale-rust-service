//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub content: String,
    pub channel_id: Uuid,
    pub participant_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel::Entity",
        from = "Column::ChannelId",
        to = "super::channel::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Channel,
    #[sea_orm(
        belongs_to = "super::participant::Entity",
        from = "Column::ParticipantId",
        to = "super::participant::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Participant,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl Related<super::participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
