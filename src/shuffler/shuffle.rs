use super::head;
use crate::config::Config;
use crate::io;
use crate::io::*;
use crate::shuffle::*;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use tempfile::NamedTempFile;

struct TmpFile {
    remaining_row_count: usize,
    file: NamedTempFile,
}

pub fn shuffle(conf: &Config) {
    let mut reader_dyn: Box<dyn BufRead> = match &conf.source {
        Some(source) => Box::new(io::reader(source.first().unwrap())),
        None => Box::new(BufReader::new(stdin())),
    };
    let mut writer_dyn: Box<dyn Write> = match &conf.destination {
        Some(destination) => Box::new(io::writer(destination)),
        None => Box::new(BufWriter::new(stdout())),
    };
    head::forward_head(
        &mut reader_dyn,
        &mut writer_dyn,
        conf,
    );

    let mut tmp_files: Vec<TmpFile> = Vec::new();
    let mut total_rows: usize = 0;
    let mut reader_ind: usize = 0;
    loop {
        let (rows, size) = read_line_with_bytes(&mut reader_dyn, conf.buffer_size, conf.feed);
        match &conf.source {
            Some(source) => {
                if reader_ind >= source.len() {
                    break;
                }
                if size == 0 {
                    reader_ind += 1;
                    if reader_ind < source.len() {
                        reader_dyn = Box::new(io::reader(&source[reader_ind]));
                        head::skip_head(&mut reader_dyn, conf);
                    }
                    continue;
                }
            }
            None => {
                if size == 0 {
                    break;
                }
            }
        }

        let file = if let Some(tmp) = &conf.tmp {
            NamedTempFile::new_in(tmp).unwrap()
        } else {
            NamedTempFile::new().unwrap()
        };
        info!("{:?}", file.path().to_str());
        let shuf: Vec<usize> = fisher_yates_shuffle_n(rows.len());
        let mut tmp_writer = io::writer(file.path().to_str().unwrap());
        for i in shuf {
            tmp_writer.write_all(rows[i].as_bytes()).unwrap();
        }
        tmp_files.push(TmpFile {
            remaining_row_count: rows.len(),
            file,
        });
        total_rows += rows.len();
    }
    info!("finished writing to tmp files, count: {}", tmp_files.len());
    let mut tmp_file_readers: Vec<BufReader<File>> = Vec::with_capacity(tmp_files.len());
    for tmp_file in tmp_files.iter() {
        tmp_file_readers.push(io::reader(tmp_file.file.path().to_str().unwrap()));
    }
    let mut rng = thread_rng();
    let writer = &mut writer_dyn;
    for i in 0..total_rows {
        let r: usize = rng.gen_range(0, total_rows - i) + 1;
        let mut current_rows = 0;
        for j in 0..tmp_files.len() {
            current_rows += tmp_files[j].remaining_row_count;
            if r <= current_rows {
                tmp_files[j].remaining_row_count -= 1;
                let mut buf = String::new();
                let size =
                    io::read_line_with_linefeed(&mut tmp_file_readers[j], &mut buf, conf.feed).expect("readline err");
                if size == 0 {
                    panic!("invalid EOF detected while reading tmp file!");
                }
                writer.write_all(buf.as_bytes()).unwrap();
                break;
            }
        }
    }
    info!("finished writing to destination file");
}
