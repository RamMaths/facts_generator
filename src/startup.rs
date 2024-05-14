use clap::Parser;
use std::str::FromStr;
use crate::format;
use std::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    //file path
    #[arg(short, long)]
    pub input_file: String,

    //file path
    #[arg(long)]
    pub fact: String,

    // output file
    #[arg(short, long)]
    pub output: String,

    //delimiter
    #[arg(short, long, default_value_t = b',')]
    pub delimiter: u8,
    
    //columns in the csv
    #[arg(short, long, value_parser = clap::value_parser!(Columns))]
    pub columns: Columns,

    //delimiter
    #[arg(long)]
    pub max: Option<usize>
}

#[derive(Clone)]
pub struct Columns(pub Vec<usize>);

impl FromStr for Columns {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns: Vec<usize> = s
        .split(',')
        .map(|part| part.parse::<usize>().expect("Couldn't parse the columns correctly"))
        .collect();

        Ok(Columns(columns))
    }
}

pub fn build() -> Args {
    Args::parse()
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = build();
    format::write_file(&args)?;

    Ok(())
}
