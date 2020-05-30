use crate::schema::transactions;
use crate::schema::categories;
use crate::schema::transaction_category;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TransactionType {
    Expense,
    Income,
}

impl std::convert::From<String> for TransactionType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "expense" => Self::Expense,
            "income" => Self::Income,
            _ => {
                eprintln!("ERR: TransactionType {} not supported, fallback to Expense", s);
                Self::Expense
            },
        }
    }
}

impl std::convert::From<TransactionType> for String {
    fn from(s: TransactionType) -> String {
        match s {
            TransactionType::Expense => "Expense".to_string(),
            TransactionType::Income => "Income".to_string(),
        }
    }
}

impl diesel::Expression for TransactionType {
    type SqlType = diesel::sql_types::Text;
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(*self).as_str())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TransactionCreateRequest {
    pub trx_id: String,
    pub title: String,
    pub description: String,
    pub category_id: Option<i32>,
}

impl std::convert::Into<Transaction> for TransactionCreateRequest {
    fn into(self) -> Transaction {
        Transaction {
            id: None,
            trx_id: self.trx_id,
            title: self.title,
            description: self.description,
            created_at: None,
            updated_at: None,
        }
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
    pub parent_id: Option<i32>,
    #[diesel(deserialize_as = "String")]
    pub type_: TransactionType,
    pub icon: String,
    pub title: String,
}

#[derive(Debug, diesel::Insertable, diesel::Queryable, serde::Serialize, serde::Deserialize)]
#[table_name="transaction_category"]
pub struct TransactionCategory {
    pub transaction_id: i32,
    pub category_id: i32,
}
