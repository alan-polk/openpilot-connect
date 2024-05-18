//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "routes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub fullname: String,
    pub git_remote: Option<String>,
    pub version: Option<String>,
    pub git_branch: Option<String>,
    pub start_time: Option<DateTime>,
    #[sea_orm(column_type = "Float")]
    pub miles: f32,
    pub platform: String,
    pub public: bool,
    pub devicetype: Option<i16>,
    pub git_dirty: Option<bool>,
    pub url: String,
    pub can: bool,
    pub git_commit: Option<String>,
    pub device_dongle_id: String,
    pub create_time: i64,
    #[sea_orm(column_type = "Double")]
    pub end_lat: f64,
    #[sea_orm(column_type = "Double")]
    pub end_lng: f64,
    pub end_time: Option<DateTime>,
    pub end_time_utc_millis: i64,
    pub hpgps: bool,
    pub init_logmonotime: i64,
    pub is_preserved: bool,
    pub is_public: bool,
    #[sea_orm(column_type = "Float")]
    pub length: f32,
    pub maxcamera: i32,
    pub maxdcamera: i32,
    pub maxecamera: i32,
    pub maxlog: i32,
    pub maxqlog: i32,
    pub maxqcamera: i32,
    pub passive: bool,
    pub proccamera: i32,
    pub proclog: i32,
    pub procqcamera: i32,
    pub procqlog: i32,
    pub radar: bool,
    pub rating: Option<String>,
    #[sea_orm(column_type = "Json")]
    pub segment_end_times: JsonValue,
    #[sea_orm(column_type = "Json")]
    pub segment_numbers: JsonValue,
    #[sea_orm(column_type = "Json")]
    pub segment_start_times: JsonValue,
    pub share_exp: String,
    pub share_sig: String,
    #[sea_orm(column_type = "Double")]
    pub start_lat: f64,
    #[sea_orm(column_type = "Double")]
    pub start_lng: f64,
    pub start_time_utc_millis: i64,
    pub user_id: String,
    pub vin: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            fullname: String::default(),
            git_remote: None,
            version: None,
            git_branch: None,
            start_time: None,
            miles: 0.0,
            platform: String::default(),
            public: false,
            devicetype: None,
            git_dirty: None,
            url: String::default(),
            can: false,
            git_commit: None,
            device_dongle_id: String::default(),
            create_time: 0,
            end_lat: 0.0,
            end_lng: 0.0,
            end_time: None,
            end_time_utc_millis: 0,
            hpgps: false,
            init_logmonotime: 0,
            is_preserved: false,
            is_public: false,
            length: 0.0,
            maxcamera: -1,
            maxdcamera: -1,
            maxecamera: -1,
            maxlog: -1,
            maxqlog: -1,
            maxqcamera: -1,
            passive: false,
            proccamera: -1,
            proclog: -1,
            procqcamera: -1,
            procqlog: -1,
            radar: false,
            rating: None,
            segment_end_times: JsonValue::Array(vec![]),
            segment_numbers: JsonValue::Array(vec![]),
            segment_start_times: JsonValue::Array(vec![]),
            share_exp: String::default(),
            share_sig: String::default(),
            start_lat: 0.0,
            start_lng: 0.0,
            start_time_utc_millis: 0,
            user_id: String::default(),
            vin: String::default(),
            created_at: DateTime::from_timestamp(0, 0),
            updated_at: DateTime::from_timestamp(0, 0),
        }
    }
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
