use crate::repo::sql::base::{Sql, SqlError, SqlSucess};

use sqlx::migrate::Migrator;
use sqlx::Postgres;
use std::{env, path::Path};

struct SqlPostgres {
    database: sqlx::Pool<Postgres>,
}

impl Sql for SqlPostgres {
    async fn new() -> Result<SqlPostgres, SqlError> {
        let database_url = env::var("DATABASE_URL").unwrap();

        let sql_pool_result = sqlx::PgPool::connect(&database_url).await;

        if sql_pool_result.is_err() {
            return Err(SqlError::DatabaseError);
        }

        let sql_pool = sql_pool_result.unwrap();

        let migration_folder = Path::new("./migrations");

        let migration_result = Migrator::new(migration_folder).await;

        if migration_result.is_err() {
            return Err(SqlError::DatabaseError);
        }

        let migration_run_result = migration_result.unwrap().run(&sql_pool).await;

        if migration_run_result.is_err() {
            return Err(SqlError::DatabaseError);
        }

        return Ok(SqlPostgres { database: sql_pool });
    }
}
