#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use reqwest::Client;
use crate::models::_entities;

use crate::views;
use serde::{Deserialize, Serialize};
use axum::{
    extract::{Query, State}, Extension,
};
extern crate url;



#[derive(Deserialize)]
pub struct OneBox {
    onebox: String
}

#[derive(Serialize)]
pub struct RoutesTemplate {
    pub defined: bool,
    pub routes: Vec<_entities::routes::Model>
}
#[derive(Serialize)]
pub struct DevicesTemplate {
    pub defined: bool,
    pub devices: Vec<_entities::devices::Model>
}
#[derive(Serialize)]
pub struct BootlogsTemplate {
    pub defined: bool,
    pub bootlogs: Vec<_entities::bootlogs::Model>
}

#[derive(Serialize)]
pub struct SegmentsTemplate {
    pub defined: bool,
    pub segments: Vec<_entities::segments::Model>,
}

#[derive(Serialize, Default)]
pub struct MasterTemplate {
    pub api_host: String,
    pub ws_host: String,
    pub onebox: String,
    pub dongle_id: String,
    pub segments: Option<SegmentsTemplate>,
    pub devices: Option<DevicesTemplate>,
    pub routes: Option<RoutesTemplate>,
    pub bootlogs: Option<BootlogsTemplate>,
}

pub async fn echo(req_body: String) -> String {
    req_body
}

pub async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
    // do something with context (database, etc)
    format::text("hello")
}

pub async fn onebox_handler(
    auth: crate::middleware::auth::MyJWT,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Extension(client): Extension<Client>,
    Query(params): Query<OneBox>,
) -> Result<impl IntoResponse> {
    // Regex to match a complete canonical route name
    let re = regex::Regex::new(r"^([0-9a-z]{16})([_|/|]?([0-9]{4}-[0-9]{2}-[0-9]{2}--[0-9]{2}-[0-9]{2}-[0-9]{2}))?$").unwrap();

    let mut canonical_route_name: Option<String> = None;
    let mut dongle_id: Option<String> = None;
    let mut timestamp: Option<String> = None;

    // Check for route or dongle ID
    if let Some(caps) = re.captures(&params.onebox) {
        dongle_id = Some(caps[1].to_string());
        if let Some(ts) = caps.get(3) {
            timestamp = Some(ts.as_str().to_string());
            canonical_route_name = Some(format!("{}|{}", dongle_id.as_ref().unwrap(), timestamp.as_ref().unwrap()));
        }
    }
    let api_host = ctx.config.server.full_url().replace("http", "https");
    let ws_host = api_host.replace("3112", "3223");

    if let Some(canonical_route) = canonical_route_name {
        let mut segment_models = Some(_entities::segments::Model::find_segments_by_route(&ctx.db, &canonical_route).await?);
        if let Some(segment_models) = segment_models.as_mut() {
            segment_models.sort_by(|a, b| a.number.cmp(&b.number));
        }
    
        // Create and render master template
        let master_template = MasterTemplate { 
            dongle_id: dongle_id.unwrap_or_default(),
            segments: segment_models.map(|segments| SegmentsTemplate { 
                defined: true, 
                segments 
            }), 
            onebox: params.onebox,
            api_host: api_host,
            ws_host: ws_host,
            ..Default::default()
        };
    
        views::route::admin_route(v, master_template)
    } else if let Some(d_id) = dongle_id {
        let route_models = _entities::routes::Model::find_device_routes(&ctx.db, &d_id).await?;
        //let user = _entities::users::Model::find_by_identity(&ctx.db, &auth.claims.identity).await?;
        //let device_models = _entities::devices::Model::find_all_devices(&ctx.db).await;
        //let device_model: Option<_entities::devices::Model> = _entities::devices::Model::find_device(&ctx.db, &d_id).await;
        let user_model = _entities::users::Model::find_by_identity(&ctx.db, &auth.claims.identity).await?;
        let device_models =  _entities::devices::Model::find_user_devices(&ctx.db, user_model.id).await;
        let bootlogs_models: Vec<_entities::bootlogs::Model> = _entities::bootlogs::Model::find_device_bootlogs(&ctx.db, &d_id).await?;

        let master_template: MasterTemplate = MasterTemplate { 
            routes: Some(RoutesTemplate { 
                defined: true, 
                routes: route_models 
            }), 
            devices: Some(DevicesTemplate {
                defined: true,
                devices: device_models,
            }),
            bootlogs: Some(BootlogsTemplate {
                defined: true,
                bootlogs: bootlogs_models
            }),
            onebox: params.onebox,
            api_host: api_host,
            ws_host: ws_host,
            ..Default::default()
        };

        views::route::admin_route(v, master_template)

    } else {
        let user_model = _entities::users::Model::find_by_identity(&ctx.db, &auth.claims.identity).await?; // should only get here if user is allowed
        let device_models =  _entities::devices::Model::find_user_devices(&ctx.db, user_model.id).await;
        let master_template = MasterTemplate { 
            devices: Some(DevicesTemplate {
                defined: true,
                devices: device_models
            }),
            onebox: params.onebox,
            api_host: api_host,
            ws_host: ws_host,
            ..Default::default() 
        };
        // Fallback response
        views::route::admin_route(v, master_template)
    }
}

pub async fn login(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    views::auth::login(
        v, 
        crate::views::auth::LoginTemplate { 
            api_host: ctx.config.server.full_url()
        }
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(onebox_handler))
        .add("/login", get(login))
        .add("/echo", post(echo))
}
