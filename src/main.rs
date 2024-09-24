// rcli csc -i input.csv -o output.json 

use std::path::Path;

use clap::{arg, command, Parser};

#[derive(Debug,Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts{
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug,Parser)]
enum SubCommand{
    #[command(name = "csv", about = "Show CSC, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug,Parser)]
struct CsvOpts{
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String,&'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
