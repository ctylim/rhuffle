#[macro_use]
extern crate log;
pub mod config;
pub mod io;
pub mod shuffle;
pub mod shuffler;

use crate::config::Config;

fn main() {
    let config = Config::new();
    env_logger::Builder::new().parse_filters(&config.log_level).init();
    config.show();
    shuffler::shuffle::shuffle(&config);
}
