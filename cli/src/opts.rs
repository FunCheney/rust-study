use std::path::Path;
use std::str::FromStr;
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

    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>,

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

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // 把 string 解析成其他的类型，前提是这个类型实现了 FromStr
    format.parse::<OutputFormat>()
}


impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml", 
        }
    }
}


impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("unknown output format {}", value),
        }
    }
}