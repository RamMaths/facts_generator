use std::{
    error::Error,
    fs::File
};
use crate::startup::Args;
use csv::Reader;
use csv::StringRecord;

pub fn give_format(args: &Args) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(&args.input_file)?;

    let rdr: Reader<File> = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(args.delimiter as u8)
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(file);


    let result = match args.max {
        Some(max) => {
            max_case(rdr, &args, max)?
        },
        None => {
            non_max_case(rdr, &args)?
        }
    };

    Ok(result)
}

fn max_case(mut rdr: Reader<File>, args: &Args, max: usize) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    for (i, record) in rdr.records().enumerate() {
        if i <= max - 1 {
            let record = record?;
            handle_column_iteration(record, &mut result, args);
        }
    }

    Ok(result)    
}

fn non_max_case(mut rdr: Reader<File>, args: &Args) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    for record in rdr.records() {
        let record = record?;
        handle_column_iteration(record, &mut result, args);
    }

    Ok(result)    
}

fn handle_column_iteration(record: StringRecord, result: &mut Vec<Vec<String>>, args: &Args) {
    let mut columns: Vec<String> = Vec::with_capacity(args.columns.0.len());

    for num in &args.columns.0 {
        let cell: &str = match record.get(*num - 1) {
            Some(cell) => cell,
            None => panic!("Bad input column")
        };

        columns.push(format!(r#""{}""#, cell));
    }

    result.push(columns);
}
