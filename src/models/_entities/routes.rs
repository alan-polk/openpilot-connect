//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "routes")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub canonical_route_name: String,
    pub public: bool,
    pub git_remote: Option<String>,
    pub version: Option<String>,
    pub git_branch: Option<String>,
    pub devicetype: Option<i16>,
    pub git_dirty: Option<bool>,
    pub url: String,
    pub can: bool,
    pub git_commit: Option<String>,
    pub device_dongle_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::devices::Entity",
        from = "Column::DeviceDongleId",
        to = "super::devices::Column::DongleId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Devices,
    #[sea_orm(has_many = "super::segments::Entity")]
    Segments,
}

impl Related<super::devices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Devices.def()
    }
}

impl Related<super::segments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Segments.def()
    }
}
