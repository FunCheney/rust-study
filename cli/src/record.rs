use std::collections::HashMap;
use std::fs::File;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        //  headers.iter() 使用 headers 的迭代器
        // record.iter() 使用 record 的迭代器
        // zip() 将两个迭代器合并为一个元组的迭代器 [(header, record),....]
        // collect::<Value>() 将元组的迭代器转为 json value
        let json = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json);
    }
    // 创建输出文件并将数据序列化为 JSON 写入
     let file = File::create(output)?;
     // 修复：直接使用原始数据 ret 而不是已序列化的 json 字符串
    serde_json::to_writer_pretty(file, &ret)?;
    Ok(())
}