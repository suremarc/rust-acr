use std::{borrow::Borrow, error::Error, io};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Table {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub path: String,
    pub table: String,
    pub ident: String,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub field: String,
    pub pattern: String,
    pub separator: char,
}

#[derive(Debug, Deserialize)]
pub struct Xref {
    pub field: String,
    pub table: String,
}

#[derive(Debug)]
pub enum Row {
    Table(Table),
    Field(Field),
    Template(Template),
    Xref(Xref),
}

#[derive(Debug, Deserialize)]
struct FieldIR {
    path: String,
}

#[derive(Debug)]
enum RowIR {
    Table(Table),
    Field(FieldIR),
    Template(Template),
    Xref(Xref),
}

#[derive(Debug)]
pub struct DB(Vec<Row>);

impl DB {
    fn expand() {}

    pub fn read() -> Result<DB, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(true)
            .from_reader(io::stdin());

        let mut ir = Vec::<RowIR>::new();

        let mut headers: Option<csv::StringRecord> = None;

        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);

            let ident = record.get(0).unwrap();
            let mut data = csv::StringRecord::from(record.iter().skip(1).collect::<Vec<&str>>());
            data.set_position(record.position().cloned());
            if ident == "header" {
                headers = Some(data);
            } else {
                ir.extend(match record.get(0).unwrap() {
                    "table" => Some(RowIR::Table(data.deserialize(headers.as_ref())?)),
                    "field" => Some(RowIR::Field(data.deserialize(headers.as_ref())?)),
                    "template" => Some(RowIR::Template(data.deserialize(headers.as_ref())?)),
                    "xref" => Some(RowIR::Xref(data.deserialize(headers.as_ref())?)),
                    _ => None,
                });
            };
        }

        Ok(DB(ir
            .into_iter()
            .map(|row| match row {
                RowIR::Table(t) => Row::Table(t),
                RowIR::Field(f) => Row::Field({
                    let path = f.path.clone();
                    let mut split = path.split('.');
                    Field {
                        path: f.path,
                        table: split.next().unwrap().to_string(),
                        ident: split.next().unwrap().to_string(),
                    }
                }),
                RowIR::Template(t) => Row::Template(t),
                RowIR::Xref(x) => Row::Xref(x),
            })
            .collect()))
    }
}
