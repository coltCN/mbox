use std::fs::File;

use anyhow::Result;
use docx_rs::{Docx, Paragraph, Run, Style, StyleType, TableCell, TableRow};
use serde::{Deserialize, Serialize};

use crate::Config;
mod mysql;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub comment: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub comment: String,
    pub data_type: String,
    pub nullable: bool,
    pub key: String,
    pub default: Option<String>,
    pub extra: String,
}

pub struct DbUtil {
    pub db: Box<dyn Database>,
}

// pub enum DbType {
//     Mysql,
// }

pub trait Database {
    fn get_tables(&mut self) -> Result<Vec<Table>>;
}

impl DbUtil {
    pub fn new(config: Config) -> Self {
        let db = match config.database.db_type.as_str() {
            "mysql" => Box::new(mysql::Mysql::new(config)),
            _ => panic!("not support database type"),
        };

        DbUtil { db }
    }

    pub fn get_tables(&mut self) -> Result<Vec<Table>> {
        self.db.get_tables()
    }

    pub fn gen_word(&mut self, path: &str) -> Result<()> {
        let tables = self.get_tables()?;
        let mut doc = Docx::new().add_style(
            Style::new("Heading1", StyleType::Paragraph)
                .name("Heading 1")
                .bold(),
        );
        for table in tables {
            doc = doc.add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(format!("{} ({})", table.name, table.comment)))
                    .style("Heading1"),
            );
            let mut table_row = vec![TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("字段名"))),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("类型"))),
                // TableCell::new()
                //     .add_paragraph(Paragraph::new().add_run(Run::new().add_text("大小"))),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("是否为空"))),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("默认值"))),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("备注"))),
            ])];
            for column in table.columns {
                table_row.push(TableRow::new(vec![
                    // 字段名
                    TableCell::new()
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(column.name))),
                    // 类型
                    TableCell::new().add_paragraph(
                        Paragraph::new().add_run(Run::new().add_text(column.data_type.to_string())),
                    ),
                    // TableCell::new()
                    //     .add_paragraph(Paragraph::new().add_run(Run::new().add_text(""))),

                    // 是否为空
                    TableCell::new().add_paragraph(
                        Paragraph::new().add_run(Run::new().add_text(if column.nullable {
                            "是"
                        } else {
                            "否"
                        })),
                    ),
                    // 默认值
                    TableCell::new()
                        .add_paragraph(Paragraph::new().add_run(
                            Run::new().add_text(column.default.unwrap_or("".to_string())),
                        )),
                    // 备注
                    TableCell::new().add_paragraph(
                        Paragraph::new().add_run(Run::new().add_text(column.comment.to_string())),
                    ),
                ]));
            }
            doc = doc.add_table(docx_rs::Table::new(table_row));
        }
        let file = File::create(path)?;
        doc.build().pack(file)?;
        Ok(())
    }
}
