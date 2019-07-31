use rhuffle::config::Config;
use rhuffle::io;
use rhuffle::shuffler::exec::Shuffler;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use tempfile::NamedTempFile;

fn load_iris() -> BufReader<File> {
    io::reader("./tests/data/iris.csv")
}

fn get_hash<T: Hash>(x: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

fn line_hashes(reader: &mut BufRead) -> Vec<u64> {
    let (lines, _) = io::read_line_with_bytes(reader, 4294967296);
    lines.iter().map(|x| get_hash(x)).collect()
}

#[test]
fn shuffle_without_head() {
    let mut before = line_hashes(&mut load_iris());

    let config = Config {
        shuffler: Default::default(),
        log_level: "none".to_string(),
        source: None,
        destination: None,
        buffer_size: 4294967296,
        head: 1,
    };
    let file = NamedTempFile::new().unwrap();
    let mut tmp_writer = io::writer(file.path().to_str().unwrap());
    let shuffler = Shuffler::Hard;
    shuffler.shuffle(&mut load_iris(), &mut tmp_writer, &config);
    let mut tmp_reader = io::reader(file.path().to_str().unwrap());
    let mut after = line_hashes(&mut tmp_reader);
    assert_eq!(
        get_hash(before.first().unwrap()),
        get_hash(after.first().unwrap())
    );
    assert_eq!(get_hash(before.sort()), get_hash(after.sort()));
}
