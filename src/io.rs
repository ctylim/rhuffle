use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            "LF" => Result::Ok(LineFeed::LF),
            "LF_CRLF" => Result::Ok(LineFeed::LF_CRLF),
            _ => Err(()),
        }
    }
}

pub fn reader(file_name: &str) -> BufReader<File> {
    let file = match File::open(file_name) {
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
    while current_size <= bytes {
        let mut buf = String::new();
        match read_line_with_linefeed(reader, &mut buf, feed) {
            Ok(0) => break,
            Ok(n) => {
                if !buf.ends_with('\n') {
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
    let file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("An error occurred while creating file {}: {}", file_name, e);
        }
    };
    BufWriter::new(file)
}

pub fn read_line_with_linefeed(reader: &mut dyn BufRead, buf: &mut String, feed: LineFeed) -> std::result::Result<usize, std::io::Error> {
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
    Ok(sz)
}

#[test]
fn read_line_with_linefeed_test_1() {
    let mut cursor = std::io::Cursor::new(b"lorem\nipsum\r\ndolor");
    let (v, _) = read_line_with_bytes(&mut cursor, 1000, LineFeed::LF);
    assert_eq!(v, ["lorem\n", "ipsum\r\ndolor\n"]);
}

#[test]
fn read_line_with_linefeed_test_2() {
    let mut cursor = std::io::Cursor::new(b"lorem\nipsum\r\ndolor");
    let (v, _) = read_line_with_bytes(&mut cursor, 1000, LineFeed::LF_CRLF);
    assert_eq!(v, ["lorem\n", "ipsum\r\n", "dolor\n"]);
}

#[test]
fn read_line_with_linefeed_test_3() {
    let mut cursor = std::io::Cursor::new(b"1\n2\n3-1\r\n3-2\r\n3-3\n4\n5\n6");
    let (v, _) = read_line_with_bytes(&mut cursor, 1000, LineFeed::LF);
    assert_eq!(v, ["1\n", "2\n", "3-1\r\n3-2\r\n3-3\n", "4\n", "5\n", "6\n"]);
}

#[test]
fn read_line_with_linefeed_test_4() {
    let mut cursor = std::io::Cursor::new(b"1\n2\n3-1\r\n3-2\r\n3-3\n4\n5\n6\n");
    let (v, _) = read_line_with_bytes(&mut cursor, 1000, LineFeed::LF_CRLF);
    assert_eq!(v, ["1\n", "2\n", "3-1\r\n", "3-2\r\n", "3-3\n", "4\n", "5\n", "6\n"]);
}

#[test]
fn read_line_with_linefeed_test_5() {
    let mut cursor = std::io::Cursor::new(b"1\n2\n3-1\r\n3-2\r3-3\n4\n5\n6\n");
    let (v, _) = read_line_with_bytes(&mut cursor, 1000, LineFeed::LF_CRLF);
    assert_eq!(v, ["1\n", "2\n", "3-1\r\n", "3-2\r3-3\n", "4\n", "5\n", "6\n"]);
}
