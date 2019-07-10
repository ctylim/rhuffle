#[macro_use]
extern crate log;

pub mod config;
pub mod io;
pub mod shuffle;

use crate::config::Config;

use crate::io::*;
use crate::shuffle::*;
use std::io::Write;

fn main() {
    let config = Config::new();
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();
    config.show();
    let mut reader = reader(&config.source);
    let mut writer = writer(&config.destination);
    loop {
        let (rows, size) = read_line_with_bytes(&mut reader, config.buffer_size);
        if size == 0 {
            break;
        }
        let shuf: Vec<usize> = fisher_yates_shuffle_n(rows.len());
        for i in shuf {
            writer.write(format!("{}", rows[i]).as_bytes()).unwrap();
        }
    }
}
