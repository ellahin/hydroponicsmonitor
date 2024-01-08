pub enum SqlError {
    DatabaseError,
    DoesnotExist,
}

pub enum SqlSucess {
    DatabaseMigrated,
}

pub trait Sql {
    async fn new() -> Result<Self, SqlError>;
}
