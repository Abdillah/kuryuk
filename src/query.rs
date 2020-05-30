use crate::diesel::RunQueryDsl;

#[derive(diesel::Queryable,diesel::QueryableByName)]
struct SingleColumn {
    #[sql_type = "diesel::sql_types::Integer"]
    pub value: i32,
}

pub fn last_insert_rowid(conn: &diesel::sqlite::SqliteConnection) -> i32 {
    let last_id: std::vec::Vec<SingleColumn> = diesel::sql_query("SELECT last_insert_rowid() as value").load(conn).unwrap();
    match last_id.first() {
        Some(row) => row.value,
        None => 0,
    }
}
