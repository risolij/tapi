pub trait Schema {
    type Id: Send;

    fn sql_id(&self) -> Self::Id;
    fn sql_select() -> &'static str;
    fn sql_select_by_id() -> &'static str;
    fn sql_insert() -> &'static str;
    fn sql_update() -> &'static str;
    fn sql_delete() -> &'static str;
}
