use std::fs::File;
// cli csv -i input.csv -o output json —header -d ','
use clap::Parser;
use cli::{Opts, Record, Subcommand};
use csv::Reader;
use serde::Deserialize;

fn main() -> anyhow::Result<()>  {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            // let records = reader.records()
            // .map(|r| r.unwrap())
            //     .collect::<Vec<StringRecord>>();
            for result in reader.deserialize() {
                let record: Record = result?;
                ret.push(record);
            }
            // 创建输出文件并将数据序列化为 JSON 写入
            let file = File::create(opts.output)?;
            // 修复：直接使用原始数据 ret 而不是已序列化的 json 字符串
            serde_json::to_writer_pretty(file, &ret)?;
        }
    }
    println!("Hello, world!");

    Ok(())
}
