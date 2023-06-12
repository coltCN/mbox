use std::fs::File;

use anyhow::{Ok, Result};
use docx_rs::{
    Docx, Paragraph, Run, RunFonts, Style, StyleType, Table, TableCell, TableProperty, TableRow,
};
fn main() -> Result<()> {
    let file = File::create("targe/hello.docx").unwrap();
    let table_row = vec![TableRow::new(vec![
        TableCell::new()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("列名").style("front"))),
        TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("类型"))),
        TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("是否为空"))),
    ])];
    let table = Table::new(table_row).style("Table1");
    let front_style = Style::new("front", StyleType::Character)
        .color("red")
        .fonts(RunFonts::new().ascii("宋体").east_asia("宋体"));
    let table_style = Style::new("Table1", StyleType::Table)
        .table_align(docx_rs::TableAlignmentType::Center)
        .table_property(
            TableProperty::new()
                .width(100, docx_rs::WidthType::Pct)
                .style("front"),
        )
        // .color("red")
        .style("front")
        .width(100, docx_rs::WidthType::Pct);
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello World!")))
        .add_table(table)
        .add_style(table_style)
        .add_style(front_style)
        .build()
        .pack(file)?;
    Ok(())
}
