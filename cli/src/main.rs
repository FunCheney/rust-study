// cli csv -i input.csv -o output json —header -d ','
use clap::Parser;
use cli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()>  {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            }else {
                "output.json".to_string()
            };
            process_csv(&opts.input, output, opts.format)?
        }
    }
    println!("Hello, world!");

    Ok(())
}
