# rhuffle

[![crates.io](https://img.shields.io/crates/v/rhuffle.svg)](https://crates.io/crates/rhuffle)
[![Build Status](https://travis-ci.org/ctylim/rhuffle.svg?branch=master)](https://travis-ci.org/ctylim/rhuffle)

**rhuffle** is a random shuffler for large file with many lines which can exceed available RAM.

**rhuffle** supports:
- shuffling huge files which does not fit in memory
- skipping head lines which should not include for shuffling (e.g. csv/tsv)
- multiple file input and flexible input formats
- rhuffle works very fast (see [benchmark results](#benchmarks).)

![rhuffle_demo](https://user-images.githubusercontent.com/10000776/85710260-0b52c900-b721-11ea-8669-974d41e56727.gif)

## Installation

See [lib.rs](https://lib.rs/install/rhuffle).

## Usage

```
USAGE:
    rhuffle [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --buf <NUMBER>
            Sets buffer size which is smaller than available RAM with bytes (default: 4294967296).

        --dst <PATH>
            Sets destination file path. If not set, destination sets to stdout. (default: None)

        --feed <LF|LF_CRLF>                        Sets acceptable line feed as EOL (default: LF_CRLF).
    -h, --head <NUMBER>
            Sets first `n` lines without shuffling (default: 0). For multiple input sources, take README a look.

        --log <off|error|warn|info|debug|trace>    Sets log level. (default: off)
        --src <[PATH]>
            Sets source file paths (space separated). If not set, source sets to stdin. (default: None)
```

### `--head n` Option
- For multiple input sources, first `n` lines in the first input source forwards to output source without shuffling.
- For second input source and later, first `n` lines in the first input source are skipped.
- Here is an example below:

in1.txt
```
head1-1
head2-1
line1-1
line2-1
```

in2.txt
```
head1-2
head2-2
line1-2
line2-2
```

```
$ rhuffle --src in1.txt in2.txt --dst out.txt --head 2
```

out.txt
```
head1-1 // L1-L2: fixed
head2-1 
line2-1 // L3-L6: shuffled globally
line1-2
line2-2
line1-1
```

### `--feed` Option
- LF_CRLF(default): accepts LF or CRLF as newline
- LF: accepts only LF as newline
- No option for CR
 
## Benchmarks

The results shown below are focused on execution time in a limited memory space.
Two datasets are used for testing.

- [Kaggle competition dataset](https://www.kaggle.com/c/new-york-city-taxi-fare-prediction/data) (New York City Taxi Fare Prediction)
- (self-owned) custom dataset

Three softwares are used for performance comparison.

- GNU shuf 
    - command: `shuf {src} -o {dst}`
- [terashuf](https://github.com/alexandres/terashuf) 
    - command: `terashuf < {src} > {dst}`
- rhuffle
    - command: `rhuffle --src {src} --dst {dst}`

Benchmarks are executed on MacBook Pro 2017, Core i7 3.1GHz, RAM 16GB.
Execution time is measured by `time`.

### Kaggle competition dataset

5.3GB size, 55423856 lines

|Software|real|user|sys|
|---|---|---|---|
|GNU shuf|0m59s|0m34s|0m14s|
|terashuf|5m06s|4m43s|0m14s|
|rhuffle|1m56s|1m06s|0m40s|

### Custom dataset

9.0GB size, 21550072 lines

|Software|real|user|sys|
|---|---|---|---|
|GNU shuf|x|x|x|
|terashuf|8m12s|7m16s|0m31s|
|rhuffle|1m47s|0m39s|0m51s|

GNU shuf was impossible to measure (very slow).