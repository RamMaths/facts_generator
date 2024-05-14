use crate::startup::Args;
use std::{
    fs,
    io::Write,
    error::Error
};

pub fn write_file(args: &Args) -> Result<(), Box::<dyn Error>> {
    // openning the file
    let mut file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(&args.output)
        {
        Ok(file) => file,
        Err(err) => {
            panic!("Error, couldn't open the file. {}", err.to_string());
        }
    };

    let result = crate::csv::give_format(args)?;

    // escribir archivo
    for h in &result {
        file.write_all(format!("{}({}).\n", &args.fact, h.join(",")).as_bytes()).expect("Couldn't write the file");
    }

    file.flush()?;

    Ok(())
}

