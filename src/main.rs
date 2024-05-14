use facts::startup;

fn main() {
    match startup::run() {
        Ok(()) => println!("All done!!"),
        Err(err) => println!("Error: {}", err)
    }
}
