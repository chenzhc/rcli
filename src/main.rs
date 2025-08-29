
// rcli csv -i input.csv -o output.json  --header --d ',' 

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Options {

    #[command(subcommand)]
    cmd: SubCommand,
}


#[derive(Debug,Subcommand)]
enum SubCommand {

    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
struct CsvOpts {

    #[arg(short, long, help = "Input file path")]
    input: String,

    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    output: String,

    #[arg(short, long, help = "Delimiter")]
    delimiter: char,

    #[arg(short, long, help = "CSV has header or not", default_value_t = true)]
    header: bool,

}

fn main() {
    println!("Hello, rust!");
}
