use std::fs;
use std::io;
use std::path;

pub fn open<T: AsRef<path::Path>>(filename: T) -> std::io::Result<io::BufReader<fs::File>> {
    Ok(io::BufReader::new(fs::File::open(filename)?))
}

pub fn remove_empty_lines(r: &Result<String, io::Error>) -> bool {
    match r {
        Err(_) => true,
        Ok(l) => !l.is_empty(),
    }
}
