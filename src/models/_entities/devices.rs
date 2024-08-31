//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub dongle_id: String,
    pub serial: String,
    pub imei: String,
    pub imei2: String,
    pub public_key: String,
    pub sim_id: Option<String>,
    pub prime: bool,
    pub prime_type: i16,
    pub online: bool,
    pub last_athena_ping: i64,
    pub uploads_allowed: bool, // TODO should be ignore_uploads
    pub owner_id: Option<i32>,
    pub device_type: String,
    pub alias: String,
    pub server_storage: i64,
    pub locations: Option<serde_json::Value>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::authorized_users::Entity")]
    AuthorizedUsers,
    #[sea_orm(has_many = "super::bootlogs::Entity")]
    Bootlogs,
    #[sea_orm(has_many = "super::routes::Entity")]
    Routes,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::OwnerId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::authorized_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthorizedUsers.def()
    }
}

impl Related<super::bootlogs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bootlogs.def()
    }
}

impl Related<super::routes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routes.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::authorized_users::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::authorized_users::Relation::Devices.def().rev())
    }
}
