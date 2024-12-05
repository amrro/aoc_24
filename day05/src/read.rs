use std::{fs, io, path::Path};

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

pub fn get_rules(raw: &str) -> Vec<(usize, usize)> {
    raw.lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub fn get_sequences(raw: &str) -> Vec<Vec<usize>> {
    raw.lines()
        .map(|l| {
            l.split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}
