//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "api_key_type")]
pub enum ApiKeyType {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "read_only")]
    ReadOnly,
    #[sea_orm(string_value = "read_write")]
    ReadWrite,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "organization_role")]
pub enum OrganizationRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "developer")]
    Developer,
    #[sea_orm(string_value = "owner")]
    Owner,
    #[sea_orm(string_value = "read_only")]
    ReadOnly,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "organization_tier")]
pub enum OrganizationTier {
    #[sea_orm(string_value = "free")]
    Free,
    #[sea_orm(string_value = "pro")]
    Pro,
    #[sea_orm(string_value = "standard")]
    Standard,
}
