use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineFeed {
    LF,
    LF_CRLF,
}

impl Default for LineFeed {
    fn default() -> Self {
        LineFeed::LF
    }
}

impl FromStr for LineFeed {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LF" => return Result::Ok(LineFeed::LF),
            "LF_CRLF" => return Result::Ok(LineFeed::LF_CRLF),
            _ => Err(()),
        }
    }
}

pub fn reader(file_name: &str) -> BufReader<File> {
    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("An error occurred while opening file {}: {}", file_name, e);
        }
    };
    BufReader::new(file)
}

pub fn read_line_with_bytes(reader: &mut dyn BufRead, bytes: usize, feed: LineFeed) -> (Vec<String>, usize) {
    let mut current_size: usize = 0;
    let mut res: Vec<String> = Vec::new();
    let mut line_buf = String::new();
    while current_size <= bytes {
        let mut buf = String::new();
        match read_line_with_linefeed(reader, &mut buf, feed) {
            Ok(0) => break,
            Ok(n) => {
                current_size += n;
                line_buf += &buf;
                res.push(line_buf);
                line_buf = String::new();
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

pub fn read_line_with_linefeed(reader: &mut dyn BufRead, buf: &mut String, feed: LineFeed) -> std::io::Result<usize> {
    let mut sz = 0;
    loop {
        let mut tbuf = String::new();
        match reader.read_line(&mut tbuf) {
            Ok(0) => break,
            Ok(n) => {
                sz += n;
                *buf += &tbuf;
                if !(feed == LineFeed::LF && tbuf.ends_with("\r\n")) {
                    return Ok(sz);
                }
            }
            Err(e) => return Err(e),
        }
    }
    return Ok(sz);
}
