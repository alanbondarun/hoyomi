use std::io::{stdin, stdout, Error, Write};

pub fn request_string(message: &str) -> Result<String, Error> {
    print!("{}", message);
    stdout().flush()?;

    let mut filepath = String::new();
    stdin()
        .read_line(&mut filepath)
        .map(|_| filepath.trim().to_string())
}
