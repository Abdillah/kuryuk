#[macro_use]
extern crate diesel;

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;
use std::default::Default;

use actix_web::{middleware, HttpRequest, HttpResponse, http::StatusCode};
use diesel::query_dsl::*;
// use diesel::prelude::*;
use diesel::connection::Connection;

mod response;
mod model;
#[macro_use]
mod schema;

use crate::response::Response;
use crate::model::Transaction;
    
pub fn diesel_connect() -> diesel::sqlite::SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    diesel::sqlite::SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn create_transaction(transaction: actix_web::web::Json<Transaction>) -> HttpResponse {
    use self::schema::transactions;

    let dbconn = diesel_connect();

    let epocmillis: i64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs().try_into().unwrap_or(0);
    let mut transaction = transaction.into_inner();
    transaction.updated_at = Some(chrono::NaiveDateTime::from_timestamp(epocmillis, 0));
    transaction.created_at = Some(chrono::NaiveDateTime::from_timestamp(epocmillis, 0));

    let insert_ops = diesel::insert_into(transactions::table)
    .values(&transaction)
    .execute(&dbconn);
    match insert_ops {
        Ok(_) => response::Data::<Transaction> {
            status: StatusCode::ACCEPTED,
            data: transaction,
        }.as_response(),
        Err(msg) => response::Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            title: String::from("Transaction cannot be saved"),
            message: msg.to_string(),
            ..Default::default()
        }.as_response(),
    }
}

async fn get_transactions(req: HttpRequest) -> HttpResponse {
    use crate::model::TransactionQuery;
    use crate::diesel::ExpressionMethods;
    use crate::schema::transactions;
    use crate::schema::transactions::dsl;

    let dbconn = diesel_connect();
    let req_id = req.match_info().get("id");

    let mut q_transactions = transactions::table.into_boxed();
    if let Some(req_id) = req_id {
        let req_id = req_id.to_string().parse::<i32>().expect("Parse error");
        q_transactions = q_transactions.filter(dsl::id.eq(req_id)).limit(1)
    }

    match q_transactions.load::<TransactionQuery>(&dbconn) {
        Ok(results) => if req_id.is_some() && results.len() > 0 {
            response::Data::<Transaction> {
                status: StatusCode::ACCEPTED,
                data: results.first().unwrap().into(),
            }.as_response()
        } else if req_id.is_none() {
            response::Data::<Vec<Transaction>> {
                status: StatusCode::ACCEPTED,
                data: results.iter().map(|x| x.into()).collect(),
            }.as_response()
        } else {
            response::Error {
                status: StatusCode::NOT_FOUND,
                title: "Transaction not found".to_string(),
                message: "Transaction not found".to_string(),
                ..Default::default()
            }.as_response()
        },
        Err(msg) => response::Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            title: msg.to_string(),
            message: msg.to_string(),
            ..Default::default()
        }.as_response(),
    }
}

async fn update_transaction() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

async fn delete_transaction() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Compress::default())
        .route("/transactions", web::get().to(get_transactions))
        .route("/transactions/{id:[0-9]{1,5}}", web::get().to(get_transactions))
        .route("/transactions", web::post().to(create_transaction))
        .route("/transactions", web::patch().to(update_transaction))
        .route("/transactions", web::delete().to(delete_transaction))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
