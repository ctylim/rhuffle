use crate::shuffler::exec::Shuffler;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub shuffler: Shuffler,
    pub log_level: String,
    pub source: String,
    pub destination: String,
    pub buffer_size: usize,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();
        let version: String = env!("CARGO_PKG_VERSION").to_string()
            + "\ncommit "
            + env!("GIT_COMMIT_HASH")
            + "\ncommit-date "
            + env!("GIT_COMMIT_DATE");
        let matches = App::new(env!("CARGO_PKG_NAME"))
            .long_version(version.as_ref())
            .author("ctylim")
            .about("rhuffle")
            .arg(
                Arg::with_name("level")
                    .short("l")
                    .long("level")
                    .value_name("hard|soft")
                    .help("Sets shuffle level. (default: hard)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("log")
                    .long("log")
                    .value_name("off|error|warn|info|debug|trace")
                    .help("Sets log level. (default: off)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("src")
                    .long("src")
                    .value_name("PATH")
                    .help("Sets source file path.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("dst")
                    .long("dst")
                    .value_name("PATH")
                    .help("Sets destination file path.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("buffer")
                    .long("buf")
                    .value_name("NUMBER")
                    .help("Sets buffer size which is approximately equivalent to available RAM with bytes (default: 3000).")
                    .takes_value(true),
            )
            .get_matches();
        let parse_failed =
            |a: &str, s: &str| format!("Parse failed in command argument {}: {}", a, s);
        if let Some(shuffler) = matches.value_of("level") {
            config.shuffler = shuffler.parse().expect(&parse_failed("shuffler", shuffler));
        }
        if let Some(log_level) = matches.value_of("log") {
            config.log_level = log_level
                .parse()
                .expect(&parse_failed("log_level", log_level));
        }
        if let Some(source) = matches.value_of("src") {
            config.source = source.parse().expect(&parse_failed("source", source));
        }
        if let Some(destination) = matches.value_of("dst") {
            config.destination = destination
                .parse()
                .expect(&parse_failed("destination", destination));
        }
        if let Some(buffer_size) = matches.value_of("buffer") {
            config.buffer_size = buffer_size
                .parse()
                .expect(&parse_failed("buffer_size", buffer_size));
        }
        config
    }

    pub fn show(&self) {
        info!("{:?}", self);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shuffler: Default::default(),
            log_level: "off".to_string(),
            source: "".to_string(),
            destination: "".to_string(),
            buffer_size: 3000,
        }
    }
}
