#[macro_use]
extern crate diesel;

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;
use std::default::Default;

use actix_web::{middleware, HttpRequest, HttpResponse, http::StatusCode};
use diesel::prelude::*;
use diesel::connection::Connection;

mod response;
mod model;
#[macro_use]
mod schema;
mod query;

use crate::response::Response;
use crate::model::Transaction;

pub fn diesel_connect() -> diesel::sqlite::SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    diesel::sqlite::SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn get_transactions(req: HttpRequest) -> HttpResponse {
    use crate::model::Model;

    let dbconn = diesel_connect();
    let req_id = req.match_info().get("id");

    if let Some(req_id) = req_id {
        // Single entity request
        let req_id = req_id.to_string().parse::<i32>().expect("Parse error");
        match Transaction::find(&dbconn, req_id) {
            Ok(transaction) => if let Some(transaction) = transaction {
                response::Data::<model::Transaction> {
                    status: StatusCode::OK,
                    data: transaction,
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
    } else {
        // Multiple entity request
        match Transaction::get(&dbconn) {
            Ok(transactions) => response::Data::<Vec<model::Transaction>> {
                status: StatusCode::OK,
                data: transactions,
            }.as_response(),
            Err(msg) => response::Error {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                title: msg.to_string(),
                message: msg.to_string(),
                ..Default::default()
            }.as_response(),
        }
    }
}

async fn create_transaction(body: actix_web::web::Json<model::TransactionCreateRequest>) -> HttpResponse {
    use self::schema::transactions;
    use self::schema::transaction_category;
    use self::schema::categories;

    let dbconn = diesel_connect();

    let cat_id = body.category_id.clone();

    let epocmillis: i64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs().try_into().unwrap_or(0);
    let mut transaction: model::Transaction = body.into_inner().into();
    transaction.updated_at = Some(chrono::NaiveDateTime::from_timestamp(epocmillis, 0));
    transaction.created_at = Some(chrono::NaiveDateTime::from_timestamp(epocmillis, 0));

    let insert_ops = diesel::insert_into(transactions::table)
    .values(&transaction)
    .execute(&dbconn);
    let last_id = query::last_insert_rowid(&dbconn);

    if let Some(category_id) = cat_id {
        let categories: Vec<model::Category> = categories::dsl::categories.filter(categories::dsl::id.eq(category_id))
        .limit(1)
        .load(&dbconn)
        .unwrap();
        if let Some(category) = categories.first() {
            if let Some(cat_id) = category.id {
                diesel::insert_into(transaction_category::table)
                .values(&model::TransactionCategory {
                    transaction_id: last_id,
                    category_id: cat_id,
                })
                .execute(&dbconn)
                .expect("Failed to create transaction_category");
            }
        }
    }

    match insert_ops {
        Ok(_) => response::Data::<model::Transaction> {
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

async fn update_transaction(req: HttpRequest, transaction: actix_web::web::Json<model::Transaction>) -> HttpResponse {
    use crate::schema::transactions::dsl;

    let req_id = req.match_info().get("id");
    if req_id.is_none() {
        return response::Error {
            status: StatusCode::BAD_REQUEST,
            title: "Parameter `id` not defined".to_string(),
            message: "Parameter `id` not defined".to_string(),
            ..Default::default()
        }.as_response();
    }
    let req_id = req_id.unwrap().to_string().parse::<i32>().expect("Parse error");

    let dbconn = diesel_connect();
    match diesel::update(dsl::transactions.find(req_id))
    .set((dsl::title.eq(transaction.title.clone()), dsl::description.eq(transaction.description.clone())))
    .execute(&dbconn) {
        Ok(_) => response::Data::<Vec<model::Transaction>> {
            status: StatusCode::OK,
            data: vec!(),
        }.as_response(),
        Err(msg) => response::Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            title: msg.to_string(),
            message: msg.to_string(),
            ..Default::default()
        }.as_response(),
    }
}

async fn delete_transaction(req: HttpRequest) -> HttpResponse {
    use crate::schema::transactions::dsl;

    let req_id = req.match_info().get("id");
    if req_id.is_none() {
        return response::Error {
            status: StatusCode::BAD_REQUEST,
            title: "Parameter `id` not defined".to_string(),
            message: "Parameter `id` not defined".to_string(),
            ..Default::default()
        }.as_response();
    }
    let req_id = req_id.unwrap().to_string().parse::<i32>().expect("Parse error");

    let dbconn = diesel_connect();
    match diesel::delete(dsl::transactions.find(req_id)).execute(&dbconn) {
        Ok(_) => response::Data::<Vec<Transaction>> {
            status: StatusCode::OK,
            data: vec!(),
        }.as_response(),
        Err(msg) => response::Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            title: msg.to_string(),
            message: msg.to_string(),
            ..Default::default()
        }.as_response(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Compress::default())
        .route("/transactions", web::get().to(get_transactions))
        .route("/transactions", web::post().to(create_transaction))
        .route("/transactions/{id:[0-9]{1,5}}", web::get().to(get_transactions))
        .route("/transactions/{id:[0-9]{1,5}}", web::patch().to(update_transaction))
        .route("/transactions/{id:[0-9]{1,5}}", web::delete().to(delete_transaction))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
