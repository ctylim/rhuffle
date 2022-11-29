use crate::config::Config;
use std::io::{BufRead, Write};

pub fn forward_head(reader: &mut dyn BufRead, writer: &mut dyn Write, conf: &Config) {
    if conf.head > 0 {
        info!("forwarding head {} lines", conf.head);
    }
    for i in 0..conf.head {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(0) => {
                panic!("EOF detected while reading head {}-th line", i);
            }
            Ok(_) => {
                writer.write_all(buf.as_bytes()).unwrap();
            }
            Err(e) => {
                panic!("An error occurred while reading line: {}", e);
            }
        }
    }
}

pub fn skip_head(reader: &mut dyn BufRead, conf: &Config) {
    if conf.head > 0 {
        info!("skipping head {} lines", conf.head);
    }
    for i in 0..conf.head {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(0) => {
                panic!("EOF detected while reading head {}-th line", i);
            }
            Ok(_) => {
                // DO NOTHING
            }
            Err(e) => {
                panic!("An error occurred while reading line: {}", e);
            }
        }
    }
}
