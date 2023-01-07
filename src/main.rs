use axum::{
    routing::post,
    Router,
    Server,
    extract::Json
};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(handle_transaction));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn handle_transaction(Json(req): Json<GatewayRequest>) -> Json<GatewayResponse> {
    todo!()
}

#[derive(PartialEq, Clone, Default, Deserialize)]
pub struct GatewayRequest {
    requesttypedescriptions: Vec<String>,
    accounttypedescription: String,
    paymenttypedescription: String,
    baseamount: u32,
    currencyiso3a: String,
    pan: String,
    expirydata: String,
    securitycode: String
}

#[derive(PartialEq, Clone, Default, Serialize)]
pub struct GatewayResponse {
    errorcode: Option<u32>,
    errormessage: Option<String>,
    requesttypedescriptions: Vec<String>,
    accounttypedescription: String,
    paymenttypedescription: String,
    baseamount: u32,
    currencyiso3a: String,
    maskedpan: String
}
