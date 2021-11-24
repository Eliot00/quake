use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::Deserialize;
use serde_derive::{Serialize};
use serde_yaml::Value;
use walkdir::{DirEntry, WalkDir};

use crate::CustomEntry;

pub struct CsvProcessor {
    pub entry: CustomEntry,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CsvTable {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>
}

impl Default for CsvTable {
    fn default() -> Self {
        CsvTable {
            header: vec![],
            rows: vec![]
        }
    }
}

impl CsvProcessor {
    pub fn read(path: PathBuf) -> Result<CsvTable, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .from_reader(file);

        let mut table = CsvTable::default();
        for record in rdr.headers() {
            for str in record {
                table.header.push(String::from(str))
            }
        }

        for result in rdr.records() {
            let record = result?;
            let mut row = vec![];
            for str in &record {
                row.push(String::from(str));
            }
            table.rows.push(row);
        }

        Ok(table)
    }

    pub fn write(_path: PathBuf, header: Vec<String>, body: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(io::stdout());

        wtr.write_record(&header)?;

        for column in body {
            wtr.write_record(&column)?;
        }

        wtr.flush()?;

        Ok(())
    }

    /// scan all entries files, and rebuild indexes
    pub fn rebuild(path: PathBuf) -> (Vec<String>, Vec<Vec<String>>) {
        fn is_markdown(entry: &DirEntry) -> bool {
            entry.file_name()
                .to_str()
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
        }

        let mut files = vec![];
        for entry in WalkDir::new(path).into_iter()
            .filter_map(|e| e.ok()) {
            if is_markdown(&entry) {
                files.push(entry.into_path());
            }
        }

        let mut header: Vec<String> = vec![];
        let mut body: Vec<Vec<String>> = vec![];
        let has_first = false;

        for file in files {
            let string = fs::read_to_string(file).expect("cannot read file");
            match CsvProcessor::entry_from_markdown(string) {
                None => {}
                Some(map) => {
                    let mut first_header: Vec<String> = vec![];
                    let mut column: Vec<String> = vec![];
                    for (key, value) in map {
                        first_header.push(key);
                        column.push(value);
                    }

                    if !has_first {
                        header = first_header;
                    }

                    body.push(column)
                }
            }
        }

        (header, body)
    }

    /// from markdown file, to parse front matter
    pub fn entry_from_markdown(text: String) -> Option<IndexMap<String, String>> {
        if !text.starts_with("---") {
            return None;
        }

        let split_data = text.split("---").map(Into::into).collect::<Vec<String>>();
        let front_matter = split_data.get(1).expect("parse issue");

        let mut map: IndexMap<String, String> = IndexMap::new();
        for document in serde_yaml::Deserializer::from_str(front_matter) {
            let value = Value::deserialize(document).expect("cannot deserialize");
            if let Value::Mapping(mapping) = value {
                for (v_key, v_value) in mapping {
                    let key = CsvProcessor::from_value(v_key);
                    let value = CsvProcessor::from_value(v_value);
                    map.insert(key, value);
                }
            }
        }

        Some(map)
    }

    pub fn from_value(value: Value) -> String {
        match value {
            Value::Null => { "".to_string() }
            Value::Bool(bool) => { bool.to_string() }
            Value::Number(num) => { num.to_string() }
            Value::String(string) => { string }
            Value::Sequence(seq) => {
                let seq = seq.into_iter()
                    .map(|value| { CsvProcessor::from_value(value) })
                    .collect::<Vec<String>>();

                seq.join(",")
            }
            Value::Mapping(_) => {
                "todo: mapping".to_string()
            }
        }
    }

    /// update in column
    pub fn update_by_column() {}
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::entry_process::csv_processor::CsvProcessor;

    #[test]
    fn read_csv() {
        let buf = PathBuf::from("_fixtures").join("todo").join("entrysets.csv");
        match CsvProcessor::read(buf) {
            Ok(table) => {
                println!("{:?}", table);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[test]
    fn rebuild() {
        let source = PathBuf::from("_fixtures");
        let buf = PathBuf::from("_fixtures").join("todo");
        let map = CsvProcessor::rebuild(buf);
        let _ = CsvProcessor::write(source, map.0, map.1);
    }

    #[test]
    fn entry_parse() {
        let text = "---
title: hello, world
authors: Phodal HUANG<h@phodal.com>
description: a hello, world
created_date: 2021.11.23
updated_date: 2021.11.21
---

sample

";

        let map = CsvProcessor::entry_from_markdown(String::from(text)).expect("parse error");
        assert_eq!("hello, world", map.get("title").unwrap());
        assert_eq!("Phodal HUANG<h@phodal.com>", map.get("authors").unwrap());
        assert_eq!("a hello, world", map.get("description").unwrap());
        assert_eq!("2021.11.23", map.get("created_date").unwrap());
        assert_eq!("2021.11.21", map.get("updated_date").unwrap());
    }
}