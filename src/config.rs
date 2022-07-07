use crate::io::LineFeed;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub log_level: String,
    pub source: Option<Vec<String>>,
    pub destination: Option<String>,
    pub buffer_size: usize,
    pub head: usize,
    pub feed: LineFeed,
    pub tmp: Option<String>,
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
                Arg::with_name("log")
                    .long("log")
                    .value_name("off|error|warn|info|debug|trace")
                    .help("Sets log level. (default: off)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("src")
                    .long("src")
                    .value_name("[PATH]")
                    .help("Sets source file paths (space separated). If not set, source sets to stdin. (default: None)")
                    .takes_value(true)
                    .min_values(0),
            )
            .arg(
                Arg::with_name("dst")
                    .long("dst")
                    .value_name("PATH")
                    .help("Sets destination file path. If not set, destination sets to stdout. (default: None)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("buffer")
                    .short("b")
                    .long("buf")
                    .value_name("NUMBER")
                    .help("Sets buffer size which is smaller than available RAM with bytes (default: 4294967296).")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("head")
                    .short("h")
                    .long("head")
                    .value_name("NUMBER")
                    .help("Sets first `n` lines without shuffling (default: 0). For multiple input sources, take README a look.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("feed")
                    .long("feed")
                    .value_name("LF|LF_CRLF")
                    .help("Sets acceptable line feed as EOL (default: LF_CRLF).")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("tmp")
                    .long("tmp")
                    .value_name("PATH")
                    .help("Sets temporary file directory. (default: Temporary directory set by the system.)")
                    .takes_value(true),
            )
            .get_matches();
        let parse_failed = |a: &str, s: &str| format!("Parse failed in command argument {}: {}", a, s);
        if let Some(log_level) = matches.value_of("log") {
            config.log_level = log_level.parse().expect(&parse_failed("log_level", log_level));
        }
        if let Some(source) = matches.values_of("src") {
            config.source = Some(source.map(|x| x.parse().unwrap()).collect());
        }
        if let Some(destination) = matches.value_of("dst") {
            config.destination = Some(destination.parse().expect(&parse_failed("destination", destination)));
        }
        if let Some(buffer_size) = matches.value_of("buffer") {
            config.buffer_size = buffer_size.parse().expect(&parse_failed("buffer_size", buffer_size));
        }
        if let Some(head) = matches.value_of("head") {
            config.head = head.parse().expect(&parse_failed("head", head));
        }
        if let Some(feed) = matches.value_of("feed") {
            config.feed = feed.parse().expect(&parse_failed("feed", feed));
        }
        if let Some(tmp) = matches.value_of("tmp"){
            config.tmp = Some(tmp.parse().expect(&parse_failed("tmp", tmp)));
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
            log_level: "off".to_string(),
            source: None,
            destination: None,
            buffer_size: 4 * 1024 * 1024 * 1024,
            head: 0,
            feed: LineFeed::LF_CRLF,
            tmp: Some(std::env::temp_dir().to_string_lossy().into_owned()),
        }
    }
}
