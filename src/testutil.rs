use std::io;

pub fn string_reader(s: &str) -> io::BufReader<&[u8]> {
    io::BufReader::new(s.as_bytes())
}
