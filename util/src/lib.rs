use std::{
    fs,
    io::{self, Read},
    path::Path,
};

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?; // Open the file
    let mut contents = String::new(); // Create an empty String to store the contents
    file.read_to_string(&mut contents)?; // Read the file's contents into the String
    Ok(contents) // Return the String
}
