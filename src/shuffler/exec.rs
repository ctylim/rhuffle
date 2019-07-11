use super::{hard, soft};
use crate::config::Config;
use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead};
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Shuffler {
    Soft,
    Hard,
}

impl Default for Shuffler {
    fn default() -> Shuffler {
        Shuffler::Hard
    }
}

impl FromStr for Shuffler {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "soft" => Result::Ok(Shuffler::Soft),
            "hard" => Result::Ok(Shuffler::Hard),
            _ => panic!("Shuffler parse error."),
        }
    }
}

impl Shuffler {
    pub fn shuffle(
        &self,
        reader: &mut BufReader<File>,
        writer: &mut BufWriter<File>,
        conf: &Config,
    ) {
        if conf.head > 0 {
            info!("forwarding head {} lines", conf.head);
        }
        for i in 0..conf.head {
            let mut buf = String::new();
            match reader.read_line(&mut buf) {
                Ok(0) => {
                    panic!("EOF detected while reading head {}-th line", i);
                },
                Ok(_) => {
                    writer.write(format!("{}", buf).as_bytes()).unwrap();
                },
                Err(e) => {
                    panic!("An error occurred while reading line: {}", e);
                }
            }
        }
        match self {
            Shuffler::Soft => {
                soft::shuffle(reader, writer, &conf);
            }
            Shuffler::Hard => {
                hard::shuffle(reader, writer, &conf);
            }
        }
    }
}
