use crate::schema::transactions;
use crate::schema::categories;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TransactionType {
    Expense,
    Income,
}

impl diesel::Expression for TransactionType {
    type SqlType = diesel::sql_types::Text;
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Expense => "TransactionType::Expense",
            Self::Income => "TransactionType::Income",
        })
    }
}

#[derive(diesel::Queryable, diesel::Insertable, serde::Serialize, serde::Deserialize, Debug)]
#[table_name="transactions"]
pub struct Transaction {
    pub id: Option<i32>, // None on unsaved
    pub trx_id: String,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub created_at: Option<chrono::NaiveDateTime>, // None on unsaved
    #[serde(skip_serializing, skip_deserializing)]
    pub updated_at: Option<chrono::NaiveDateTime>, // None on unsaved
}

#[derive(diesel::Insertable, diesel::Queryable, serde::Serialize, serde::Deserialize, Debug)]
#[table_name="categories"]
pub struct Category {
    pub id: Option<i32>,
    pub type_: TransactionType,
    pub icon: String,
    pub title: String,
    pub description: String,
}
