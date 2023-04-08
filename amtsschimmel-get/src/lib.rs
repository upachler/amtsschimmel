use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, Decode},
};

#[derive(Debug, Clone)]
struct Office {
    id: i32,
    name: String,
}

impl TryFrom<&pg::Row> for Office {
    type Error = anyhow::Error;

    fn try_from(row: &pg::Row) -> Result<Self, Self::Error> {
        let id = i32::decode(&row[0])?;
        let name = String::decode(&row[1])?;

        Ok(Self { id, name })
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_amtsschimmel_get(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let address = std::env::var("DB_URL_ENV")?;

    let sql = "SELECT * FROM civic_office";
    let rowset = pg::query(&address, sql, &[])?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut response_lines = vec![];

    for row in rowset.rows {
        let office = Office::try_from(&row)?;

        println!("office: {:#?}", office);
        response_lines.push(format!("office: {:#?}", office));
    }

    let response = format!(
        "Found {} office(s) as follows:\n{}\n\n(Column info: {})\n",
        response_lines.len(),
        response_lines.join("\n"),
        column_summary,
    );

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain;charset=utf-8")
        .body(Some(response.into()))?)
}

fn format_col(column: &pg::Column) -> String {
    format!("{}:{:?}", column.name, column.data_type)
}
