use std::io;


pub fn extract_headers(file_contents: &str) -> Result<Vec<String>, io::Error> {

    let mut reader = csv::Reader::from_reader(file_contents.as_bytes());

    let mut result: Vec<String> = Vec::with_capacity(reader.headers()?.len());

    for header in reader.headers()? {
        result.push(header.to_string());
    }

    Ok(result)
}