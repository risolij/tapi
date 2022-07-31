use sqlx::PgPool;

pub async fn establish_connection_pool() -> PgPool {
    let connection = std::env::var("DATABASE_URL").expect("Failed to get DB URL");
    sqlx::PgPool::connect(&connection)
        .await
        .expect("Failed to create pool")
}

pub trait Schema {
    type Id: Send;

    fn sql_id(&self) -> Self::Id;
    fn sql_select() -> &'static str;
    fn sql_select_by_id() -> &'static str;
    fn sql_insert() -> &'static str;
    fn sql_update() -> &'static str;
    fn sql_delete() -> &'static str;
}
