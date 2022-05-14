use std::io;
use std::path::Path;


pub fn extract_headers(file_path: &Path) -> Result<Vec<String>, io::Error> {
    let mut reader = csv::Reader::from_path(file_path)?;

    let mut result: Vec<String> = vec![];

    for header in reader.headers()? {
        result.push(header.to_string());
    }

    Ok(result)
}