use crate::config::Config;
use crate::io::*;
use crate::shuffle::*;
use std::io::{BufRead, Write};

pub fn shuffle(reader: &mut BufRead, writer: &mut Write, conf: &Config) {
    loop {
        let (rows, size) = read_line_with_bytes(reader, conf.buffer_size);
        if size == 0 {
            break;
        }
        let shuf: Vec<usize> = fisher_yates_shuffle_n(rows.len());
        for i in shuf {
            writer.write(format!("{}", rows[i]).as_bytes()).unwrap();
        }
    }
    info!("finished writing to destination file");
}
