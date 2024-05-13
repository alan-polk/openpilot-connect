#![allow(clippy::unused_async)]
use axum::extract::Query;
use loco_rs::prelude::*;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, encode, errors::Result as JWTResult, get_current_timestamp, Algorithm, DecodingKey,
    EncodingKey, Header, TokenData, Validation,
};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceClaims {
    register: bool,
    exp: i64,
}

///Query parameters
//
/// imei: Device IMEI
/// 
/// imei2: Device IMEI, second slot
/// 
/// serial: Device Serial
/// 
/// public_key: 2048-bit RSA Public Key
/// 
/// register_token: JWT token signed by your private key containing payload: {"register": True}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceRegistrationParams {
    pub imei: String,
    pub imei2: String,
    pub serial: String,
    pub public_key: String,
    pub register_token: String
}
impl DeviceRegistrationParams {
    pub fn generate_dongle_id(&self) -> String {
        let mut hasher = Sha256::new();

        // Concatenate the device info
        hasher.update(&self.imei);
        hasher.update(&self.imei2);
        hasher.update(&self.serial);
        hasher.update(&self.public_key);

        // Compute the hash
        let result = hasher.finalize();

        // Encode the hash in hexadecimal and trim to 16 characters
        let hex_encoded = hex::encode(result);
        hex_encoded[0..16].to_string() // Take only the first 16 characters
    }
}

/// Key	    Type    Description
/// 
///"dongle_id"	    (string)	Dongle ID
/// 
///"access_token"	(string)    JWT token (see Authentication)
#[derive(Debug, Deserialize, Serialize)]
struct PilotAuthResponse {
    dongle_id: String,
    access_token: String,
}

pub async fn echo(req_body: String) -> String {
    req_body
}

pub async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
    // do something with context (database, etc)
    format::text("hello")
}

async fn decode_register_token(params: &DeviceRegistrationParams) -> Option<TokenData<DeviceClaims>> {
    //let mut validate = Validation::new(Algorithm::RS256);
    //validate.leeway = 0;
    let claims = decode::<DeviceClaims>(
        &params.register_token,
        &DecodingKey::from_rsa_pem(&params.public_key.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    );
    match claims {
        Ok(claims) => Some(claims),
        Err(e) => None,
    }
}
pub async fn pilotauth(
    State(ctx): State<AppContext>,
    Query(params): Query<DeviceRegistrationParams>
) -> Result<Response> {
    let _token = decode_register_token(&params).await;
    format::json(PilotAuthResponse { dongle_id: params.generate_dongle_id(), access_token: "".into()})
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("v2")
        .add("/", get(hello))
        .add("/echo", post(echo))
        .add("/pilotauth", post(pilotauth))
}
