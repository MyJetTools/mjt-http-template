use my_postgres_macros::{InsertDbEntity, SelectDbEntity};

#[derive(Debug, InsertDbEntity, SelectDbEntity)]
pub struct TestDataDbModel {
    pub id: String,
    pub name: String
}