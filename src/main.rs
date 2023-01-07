mod constant;
mod errors;
mod gateway;
mod operation;
mod useful;

use crate::{errors::Error, operation::Operation};
use axum::{extract::Json, response::IntoResponse, routing::post, Router, Server};
use hyper::StatusCode;

pub async fn handle_transaction(Json(req): Json<gateway::Request>) -> impl IntoResponse {
    println!("Received: {req:#?}");
    let op = Operation::from_request(&req);
    match op {
        Ok(op) => {
            let res = gateway::Response::success(&op);
            (StatusCode::OK, Json(res))
        }
        Err(error) => match error {
            Error::FieldError(_) | Error::ValidationError(_) => {
                let res = gateway::Response::error(&error, &req);
                (StatusCode::BAD_REQUEST, Json(res))
            }
        },
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handle_transaction));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
