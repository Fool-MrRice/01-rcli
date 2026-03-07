use anyhow::Result;
use csv::Reader;
// use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::OutputFormat;

// 不需要的结构体，我们已经通过.headers()?完成了头部的解析
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]

// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
