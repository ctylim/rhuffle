#[macro_use]
extern crate log;
pub mod config;
pub mod io;
pub mod shuffle;
pub mod shuffler;

use crate::config::Config;

fn main() {
    let config = Config::new();
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();
    config.show();
    shuffler::shuffle::shuffle(&config);
}
