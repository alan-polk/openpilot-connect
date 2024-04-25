//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "segments")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub canonical_name: String,
    pub canonical_route_name: String,
    pub url: String,
    pub qlog_url: String,
    pub rlog_url: String,
    pub fcam_url: String,
    pub dcam_url: String,
    pub ecam_url: String,
    pub start_time_utc_millis: i64,
    pub create_time: Option<i32>,
    pub hpgps: Option<bool>,
    pub end_time_utc_millis: i64,
    #[sea_orm(column_type = "Float", nullable)]
    pub end_lng: Option<f32>,
    #[sea_orm(column_type = "Float", nullable)]
    pub start_lng: Option<f32>,
    pub passive: Option<bool>,
    pub proc_log: i16,
    pub git_branch: Option<String>,
    #[sea_orm(column_type = "Float", nullable)]
    pub end_lat: Option<f32>,
    pub proc_camera: Option<i16>,
    pub can: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::routes::Entity",
        from = "Column::CanonicalName",
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
