use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::path;

pub fn open(filename: &str) -> std::io::Result<io::BufReader<fs::File>> {
    Ok(io::BufReader::new(fs::File::open(runfile(filename))?))
}

pub fn remove_empty_lines(r: &Result<String, io::Error>) -> bool {
    match r {
        Err(_) => true,
        Ok(l) => !l.is_empty(),
    }
}

pub fn runfile(p: &str) -> path::PathBuf {
    match env::var("RUNFILES_DIR") {
        Ok(runfiles_path) => path::PathBuf::from(runfiles_path).join(p),
        Err(_) => path::PathBuf::from(p),
    }
}

pub fn read_runfile_to_string(filename: &str) -> std::io::Result<String> {
    let mut s = String::new();
    let mut f = open(filename)?;
    f.read_to_string(&mut s)?;
    Ok(s)
}
