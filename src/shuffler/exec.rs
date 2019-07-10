use super::{hard, soft};
use crate::config::Config;
use std::fs::File;
use std::io::{BufReader, BufWriter};
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
