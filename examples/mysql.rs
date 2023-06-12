use anyhow::Result;
use mbox::{Config, DbUtil};

#[tokio::main]
pub async fn main() -> Result<()> {
    let config = Config::from_file("fixtures/config.yaml").await?;

    let mut db = DbUtil::new(config);
    db.gen_word("target/hello.docx")?;
    // println!("{:#?}", db.get_tables()?);
    Ok(())
}
