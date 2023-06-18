use anyhow::Result;
use oracle::Connection;

use crate::{Column, Config, Table};

use super::Database;

pub struct Oracle {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Oracle {
    pub fn new(config: Config) -> Self {
        Oracle {
            host: config.database.host,
            port: config.database.port,
            user: config.database.username,
            password: config.database.password,
            database: config.database.name,
        }
    }

    pub fn get_conn(&mut self) -> Result<Connection> {
        let conn = Connection::connect(
            &self.user,
            &self.password,
            format!("{}:{}/{}", self.host, self.port, self.database).as_str(),
        )?;

        Ok(conn)
    }
}

impl From<Config> for Oracle {
    fn from(config: Config) -> Self {
        Oracle::new(config)
    }
}

impl Database for Oracle {
    fn get_tables(&mut self) -> Result<Vec<Table>> {
        let conn = self.get_conn()?;

        let tables = conn.query("select TABLE_NAME ,COMMENTS from user_tab_comments", &[])?;

        let mut col_comment_stmt = conn
            .statement("select column_name, comments from user_col_comments where table_name = :1")
            .build()?;

        let mut col_stmt = conn
            .statement("select COLUMN_NAME ,DATA_TYPE ,DATA_LENGTH ,DATA_PRECISION ,DATA_SCALE ,NULLABLE,DATA_DEFAULT from user_tab_columns where Table_Name=:1")
            .build()?;

        let tables: Vec<Table> = tables
            .map(|t| {
                let row = t.unwrap();
                let table_name: String = row.get(0).unwrap();
                let comment: String = row.get(1).unwrap_or_default();
                let mut col_comments = col_comment_stmt
                    .query_as::<(String, Option<String>)>(&[&table_name])
                    .unwrap();
                let cols = col_stmt.query(&[&table_name]).unwrap();
                let columns = cols
                    .map(|c| {
                        let row = c.unwrap();
                        let name: String = row.get(0).unwrap();
                        let data_type: String = row.get(1).unwrap();
                        let data_length: Option<i32> = row.get(2).unwrap();
                        let data_precision: Option<i32> = row.get(3).unwrap();
                        let data_scale: Option<i32> = row.get(4).unwrap();
                        let nullable: String = row.get(5).unwrap();
                        let default: String = row.get(6).unwrap_or_default();
                        let comment = col_comments
                            .find(|t| t.as_ref().unwrap().0 == name)
                            .map(|t| t.unwrap_or_default().1.clone())
                            .unwrap_or_else(|| None);
                        let data_type = if let Some(num_precision) = data_precision {
                            if let Some(num_scale) = data_scale {
                                if num_scale == 0 {
                                    format!("{}({})", data_type, num_precision)
                                } else {
                                    format!("{}({},{})", data_type, num_precision, num_scale)
                                }
                            } else {
                                format!("{}({})", data_type, num_precision)
                            }
                        } else if let Some(data_length) = data_length {
                            format!("{}({})", data_type, data_length)
                        } else {
                            data_type
                        };
                        Column {
                            name,
                            comment,
                            data_type,
                            nullable: nullable == "Y",
                            key: "".to_string(),
                            default: Some(default),
                            extra: "".to_string(),
                        }
                    })
                    .collect::<Vec<Column>>();
                Table {
                    name: table_name,
                    comment,
                    columns,
                }
            })
            .collect::<Vec<Table>>();

        Ok(tables)
    }
}
