use anyhow::Result;
use mbox::Config;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yaml");
    let config = Config::from_yaml(content)?;

    print!("{:#?}", config);
    Ok(())
}
