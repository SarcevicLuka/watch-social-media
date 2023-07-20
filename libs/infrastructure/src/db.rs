use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Create a pool of connections and connect to database
pub fn get_postgres_connection_pool() -> DbPool {
    let mut params =
        config::get_multiple_default(vec![("POSTGRES_DB_URL", ""), ("PG_POOL_MAX_SIZE", "8")]);
    
    let pool_size: u32 = params.pop().unwrap().parse().unwrap();
    let db_url = params.pop().unwrap();

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create postgres db pool: {}", e))
}

/// Struct to hold postgres connection pool
//  pool is dead code because it is not used yet
#[allow(dead_code)]
pub struct Postgres {
    pool: DbPool,
}

impl Default for Postgres {
    fn default() -> Self {
        Self::new()
    }
}

impl Postgres {
    /// Create new instance of postgres pool
    pub fn new() -> Postgres {
        Postgres { 
            pool: get_postgres_connection_pool()
        }
    }

    /// Get connection from the pool
    pub fn connection(&self) -> Result<DbConnection, error::Error> {
        self.pool.get().map_err(error::Error::from)
    }
}