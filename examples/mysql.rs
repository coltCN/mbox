use anyhow::Result;
use mbox::{Config, DbUtil};

#[tokio::main]
pub async fn main() -> Result<()> {
    let config = Config::from_file("fixtures/test.yaml").await?;

    let mut db = DbUtil::new(config);
    db.gen_word("hello.docx")?;
    Ok(())
}
