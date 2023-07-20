use super::db::{DbPool, DbConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct StaticData {
    pub postgres: DbPool,
}

#[derive(Clone)]
pub struct AppState {
    pub static_state: Arc<StaticData>,
}

impl AppState {
    pub fn postgres_connection(&self) -> DbConnection {
        match self.static_state.postgres.get() {
            Ok(connection) => connection,
            Err(e) => panic!("State: {}", e),
        }
    }
}

pub fn initialize() -> AppState {
    AppState { 
        static_state: Arc::new(StaticData { 
            postgres: super::db::get_postgres_connection_pool(),
        })
    }
}