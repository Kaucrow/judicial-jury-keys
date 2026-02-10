use sqlx::{
    postgres::{PgArguments, PgPoolOptions, PgRow},
    FromRow, PgPool, Postgres,
};

pub type DbResult<T> = Result<T, sqlx::Error>;

#[derive(Clone, Debug)]
pub struct Db {
    pool: PgPool,
}

#[allow(dead_code)]
impl Db {
    pub async fn connect(database_url: &str, max_connections: u32) -> DbResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn query<'q>(&'q self, sql: &'q str) -> sqlx::query::Query<'q, Postgres, PgArguments> {
        sqlx::query(sql)
    }

    pub fn query_as<'q, T>(
        &'q self,
        sql: &'q str,
    ) -> sqlx::query::QueryAs<'q, Postgres, T, PgArguments>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        sqlx::query_as(sql)
    }

    pub async fn fetch_one<T>(
        &self,
        query: sqlx::query::QueryAs<'_, Postgres, T, PgArguments>,
    ) -> DbResult<T>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        query.fetch_one(&self.pool).await
    }

    pub async fn fetch_many<T>(
        &self,
        query: sqlx::query::QueryAs<'_, Postgres, T, PgArguments>,
    ) -> DbResult<Vec<T>>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        query.fetch_all(&self.pool).await
    }

    pub async fn execute(
        &self,
        query: sqlx::query::Query<'_, Postgres, PgArguments>,
    ) -> DbResult<u64> {
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
