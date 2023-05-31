use std::borrow::BorrowMut;

use super::Column;
use crate::{Config, Table};
use anyhow::Result;
use mysql::prelude::*;
use mysql::Pool;

use super::Database;

pub struct Mysql {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pool: Option<Pool>,
}

impl Mysql {
    pub fn new(config: Config) -> Self {
        Mysql {
            host: config.database.host,
            port: config.database.port,
            user: config.database.username,
            password: config.database.password,
            database: config.database.name,
            pool: None,
        }
    }

    pub fn get_pool(&mut self) -> &Pool {
        if self.pool.is_none() {
            let opts = mysql::OptsBuilder::new()
                .ip_or_hostname(Some(self.host.as_str()))
                .user(Some(self.user.as_str()))
                .pass(Some(self.password.as_str()))
                .db_name(Some(self.database.as_str()));
            let pool = Pool::new(opts).unwrap();
            self.pool = Some(pool);
        }

        self.pool.as_ref().unwrap()
    }

    // pub async fn
}

impl From<Config> for Mysql {
    fn from(config: Config) -> Self {
        Mysql::new(config)
    }
}

impl Database for Mysql {
    fn get_tables(&mut self) -> Result<Vec<Table>> {
        let mut conn = self.get_pool().get_conn()?;

        let mut
        tables = conn.exec_map(
            "select table_name, table_comment from information_schema.tables where table_schema = ?",
            (self.database.as_str(),),
            |(name, comment)| Table {
                name,
                comment,
                columns: vec![],
            },
        )?;

        // 查询表字段
        for table in tables.iter_mut() {
            let columns = conn.exec_map(
                "select column_name, column_comment, data_type, is_nullable, column_key, column_default, extra from information_schema.columns where table_schema = ? and table_name = ?",
                (self.database.as_str(), table.name.as_str()),
                |row:(String,String,String,String,String ,Option<String>,String)| {
                    let (name, comment, data_type, nullable, key, default, extra) = row;
                    Column {
                        name,
                        comment,
                        data_type,
                        nullable: nullable == "YES",
                        key,
                        default,
                        extra,
                    }
                },
            )?;

            table.borrow_mut().columns = columns;
        }

        Ok(tables)
    }
}
