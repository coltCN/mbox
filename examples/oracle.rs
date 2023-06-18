use anyhow::Result;
use mbox::Config;
use oracle::Connection;

#[tokio::main]
pub async fn main() -> Result<()> {
    let config = Config::from_file("fixtures/config.yaml").await?;

    let conn = Connection::connect(
        &config.database.username,
        &config.database.password,
        format!(
            "{}:{}/{}",
            config.database.host, config.database.port, config.database.name
        )
        .as_str(),
    )?;

    print!("{:#?}", conn);

    let tables = conn.query("select TABLE_NAME ,COMMENTS from user_tab_comments", &[])?;

    let mut column_stmt = conn
        .statement("select column_name,comments from user_col_comments where table_name = :1")
        .build()?;

    for table in tables {
        let row = table?;
        let table_name: String = row.get(0)?;
        let comment: String = row.get(1).unwrap_or_default();
        println!("table_name: {},comment: {}", table_name, comment);
        let rows = column_stmt.query(&[&table_name])?;
        for row in rows {
            let row = row?;
            let column_name: String = row.get(0)?;
            let comment: String = row.get(1).unwrap_or_default();
            println!("column_name: {},comment: {}", column_name, comment);
        }
    }
    Ok(())
}
