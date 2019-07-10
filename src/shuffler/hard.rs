use crate::config::Config;
use crate::io::*;
use crate::shuffle::*;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufWriter};

pub fn shuffle(reader: &mut BufReader<File>, writer: &mut BufWriter<File>, conf: &Config) {
    unimplemented!()
}
