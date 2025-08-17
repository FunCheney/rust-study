// cli csv -i input.csv -o output json —header -d ','
use clap::Parser;
use cli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()>  {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?
        }
    }
    println!("Hello, world!");

    Ok(())
}
