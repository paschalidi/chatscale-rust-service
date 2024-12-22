//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_keys::Entity")]
    ApiKeys,
    #[sea_orm(has_many = "super::organization_members::Entity")]
    OrganizationMembers,
}

impl Related<super::api_keys::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKeys.def()
    }
}

impl Related<super::organization_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrganizationMembers.def()
    }
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        super::organization_members::Relation::Organizations.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::organization_members::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}