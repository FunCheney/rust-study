use std::fs::File;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Age")]
    pub age: u8,
}

pub fn process_csv(input_path: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input_path)?;
    let mut ret = Vec::with_capacity(128);
    // let records = reader.records()
    // .map(|r| r.unwrap())
    //     .collect::<Vec<StringRecord>>();
    for result in reader.deserialize() {
        let record: Record = result?;
        ret.push(record);
    }
    // 创建输出文件并将数据序列化为 JSON 写入
    let file = File::create(output)?;
    // 修复：直接使用原始数据 ret 而不是已序列化的 json 字符串
    serde_json::to_writer_pretty(file, &ret)?;
    Ok(())
}