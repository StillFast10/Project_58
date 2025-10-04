use tokio_postgres::{NoTls};
use deadpool_postgres::{Client, Config, Pool, ManagerConfig, RecyclingMethod};
use serde::Serialize;
use crate::error::DbError;


#[derive(Debug, Clone)]
pub struct DbConfig {
    pub db_name: String,
    pub user: String,
    pub password: String,
    pub host: String,
}

impl DbConfig{
    pub fn create_pool(config: DbConfig) -> Result<Pool, DbError>{
        let mut cfg = Config::new();
        cfg.dbname = Some(config.db_name.to_string());
        cfg.user = Some(config.user.to_string());
        cfg.password = Some(config.password.to_string());
        cfg.host = Some(config.host.to_string());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,});

        match cfg.create_pool(None, NoTls){
            Ok(pool) => {
                Ok(pool)
            },

            Err(e) => {
                Err(DbError::Pool_creation(e))
            },
        }
    }
}


