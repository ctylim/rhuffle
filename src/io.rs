use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

pub fn reader(file_name: &str) -> BufReader<File> {
    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("An error occurred while opening file {}: {}", file_name, e);
        }
    };
    BufReader::new(file)
}

pub fn read_line_with_bytes(reader: &mut BufRead, bytes: usize) -> (Vec<String>, usize) {
    let mut current_size: usize = 0;
    let mut res: Vec<String> = Vec::new();
    while current_size <= bytes {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if !buf.ends_with("\n") {
                    buf += "\n";
                }
                current_size += n;
                res.push(buf);
                trace!(
                    "current bytes: {}, total bytes: {}, length: {}",
                    n,
                    current_size,
                    res.len()
                );
            }
            Err(e) => {
                panic!("An error occurred while reading line: {}", e);
            }
        };
    }
    (res, current_size)
}

pub fn writer(file_name: &str) -> BufWriter<File> {
    let file = match File::create(&file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("An error occurred while creating file {}: {}", file_name, e);
        }
    };
    BufWriter::new(file)
}
