use crate::config::Config;
use crate::io;
use crate::io::*;
use crate::shuffle::*;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use tempfile::NamedTempFile;

struct TmpFile {
    remaining_row_count: usize,
    file: NamedTempFile,
}

pub fn shuffle(reader: &mut BufRead, writer: &mut Write, conf: &Config) {
    info!("tmp dir name: {:?}", std::env::temp_dir());
    let mut tmp_files: Vec<TmpFile> = Vec::new();
    let mut total_rows: usize = 0;
    loop {
        let (rows, size) = read_line_with_bytes(reader, conf.buffer_size);
        if size == 0 {
            break;
        }

        let file = NamedTempFile::new().unwrap();
        let shuf: Vec<usize> = fisher_yates_shuffle_n(rows.len());
        let mut tmp_writer = io::writer(file.path().to_str().unwrap());
        for i in shuf {
            tmp_writer.write(format!("{}", rows[i]).as_bytes()).unwrap();
        }
        tmp_files.push(TmpFile {
            remaining_row_count: rows.len(),
            file: file,
        });
        total_rows += rows.len();
    }
    info!("finished writing to tmp files, count: {}", tmp_files.len());
    let mut tmp_file_readers: Vec<BufReader<File>> = Vec::with_capacity(tmp_files.len());
    for tmp_file in tmp_files.iter() {
        tmp_file_readers.push(io::reader(tmp_file.file.path().to_str().unwrap()));
    }
    let mut rng = thread_rng();
    for i in 0..total_rows {
        let r: usize = rng.gen_range(0, total_rows - i) + 1;
        let mut current_rows = 0;
        for j in 0..tmp_files.len() {
            current_rows += tmp_files[j].remaining_row_count;
            if r <= current_rows {
                tmp_files[j].remaining_row_count -= 1;
                let mut buf = String::new();
                let size = tmp_file_readers[j].read_line(&mut buf).unwrap();
                if size == 0 {
                    panic!("invalid EOF detected while reading tmp file!");
                }
                writer.write(format!("{}", buf).as_bytes()).unwrap();
                break;
            }
        }
    }
    info!("finished writing to destination file");
}
