mod constant;
mod errors;
mod operation;
mod pan;
mod request;
mod response;
mod useful;

use crate::operation::Operation;
use axum::{extract::Json, response::IntoResponse, routing::post, Router, Server};
use hyper::StatusCode;
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::Uuid, ActiveModelTrait, Set};

pub async fn handle_transaction(Json(req): Json<request::Request>) -> impl IntoResponse {
    println!("Received: {req:#?}");
    let op = Operation::from_request(&req);
    if let Err(error) = op {
        let res = response::Response::error(&error, &req);
        return (StatusCode::BAD_REQUEST, Json(res));
    }
    let res = response::Response::success(&op.unwrap());
    // send to bank
    // save to db
    (StatusCode::OK, Json(res))
}

#[tokio::main]
async fn main() {
    let db = sea_orm::Database::connect("postgres://postgres:password@localhost/postgres")
        .await
        .expect("failed to connect to db");
    Migrator::up(&db, None)
        .await
        .expect("failed to apply migration at startup");
    let trx = entity::transaction::ActiveModel {
        id: Set(Uuid::new_v4()),
        base_amount: Set(3000),
        date_created: Set(chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 0, 0)
            .unwrap()),
        request_type: Set("AUTH".into()),
        account_type: Set("ECOM".into()),
        payment_type: Set("VISA".into()),
        currency_iso3a: Set("GBP".into()),
        status: Set(3),
        encrypted_pan: Set("4000********1000".into()),
        expiry_date: Set("12/2022".into()),
    };
    trx.insert(&db).await.expect("failed to insert");
    let app = Router::new().route("/", post(handle_transaction));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
