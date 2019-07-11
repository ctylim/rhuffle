#[macro_use]
extern crate log;
pub mod config;
pub mod io;
pub mod shuffle;
pub mod shuffler;

use crate::config::Config;
use std::cell::RefCell;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::rc::Rc;

fn main() {
    let config = Config::new();
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();
    config.show();
    let reader: Rc<RefCell<BufRead>> = match &config.source {
        Some(source) => Rc::new(RefCell::new(io::reader(source))),
        None => Rc::new(RefCell::new(BufReader::new(stdin()))),
    };
    let writer: Rc<RefCell<Write>> = match &config.destination {
        Some(destination) => Rc::new(RefCell::new(io::writer(destination))),
        None => Rc::new(RefCell::new(BufWriter::new(stdout()))),
    };
    config.shuffler.shuffle(
        &mut *reader.as_ref().borrow_mut(),
        &mut *writer.as_ref().borrow_mut(),
        &config,
    );
}
