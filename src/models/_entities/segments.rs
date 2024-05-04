//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "segments")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub canonical_name: String,
    pub canonical_route_name: String,
    pub number: i16,
    pub url: String,
    pub ulog_url: String,
    pub qlog_url: String,
    pub qcam_url: String,
    pub rlog_url: String,
    pub fcam_url: String,
    pub dcam_url: String,
    pub ecam_url: String,
    pub proc_camera: i32,
    pub proc_log: i32,
    pub can: bool,
    pub hpgps: bool,
    #[sea_orm(column_type = "Double")]
    pub start_lng: f64,
    #[sea_orm(column_type = "Double")]
    pub end_lng: f64,
    #[sea_orm(column_type = "Double")]
    pub start_lat: f64,
    #[sea_orm(column_type = "Double")]
    pub end_lat: f64,
    pub start_time_utc_millis: i64,
    pub create_time: Option<i32>,
    pub end_time_utc_millis: i64,
    pub passive: Option<bool>,
    pub git_branch: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::routes::Entity",
        from = "Column::CanonicalRouteName",
        to = "super::routes::Column::CanonicalRouteName",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Routes,
}

impl Related<super::routes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routes.def()
    }
}
