use std::path::Path;
use clap::Parser;

/// Parser 可以和 命令行的参数联系起来
#[derive(Debug, Parser)]
#[command(name= "cli", version, author, about, long_about = None) ]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name="csv", about = "show CSV, or Convert CSV file to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}


/// 校验方法
fn verify_file(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.into())
    }else {
        Err(format!("{} does not exist", path))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, String> {
    match format.to_lowercase().as_str() {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        "toml" => Ok(OutputFormat::Toml),
        _ => Err(format!("unknown output format {}", format)),
    }
}
