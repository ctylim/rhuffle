#[macro_use]
extern crate log;
pub mod config;
pub mod io;
pub mod shuffle;
pub mod shuffler;

use crate::config::Config;

use crate::io::*;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::rc::Rc;

fn main() {
    let config = Config::new();
    let reader: Rc<RefCell<BufReader<File>>> = Rc::new(RefCell::new(reader(&config.source)));
    let writer: Rc<RefCell<BufWriter<File>>> = Rc::new(RefCell::new(writer(&config.destination)));

    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();
    config.show();
    config.shuffler.shuffle(
        &mut *reader.as_ref().borrow_mut(),
        &mut *writer.as_ref().borrow_mut(),
        &config,
    );
}
