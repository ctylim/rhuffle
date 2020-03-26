# rhuffle

[![crates.io](https://img.shields.io/crates/v/rhuffle.svg)](https://crates.io/crates/rhuffle)
[![Build Status](https://travis-ci.org/ctylim/rhuffle.svg?branch=master)](https://travis-ci.org/ctylim/rhuffle)

**rhuffle** is a random shuffler for large file with many lines which can exceed available RAM.

## How to use as a CLI tool

```
$ cargo install rhuffle
$ rhuffle --help
```

## How to Build

```
$ cargo build --release
```

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

        --dst <Option<PATH>>
            Sets destination file path. If not set, destination sets to stdout. (default: None)

    -h, --head <NUMBER>
            Sets first `n` lines without shuffling (default: 0). For multiple input sources, please take README a look.

        --log <off|error|warn|info|debug|trace>    Sets log level. (default: off)
        --src <Option<PATHS>>
            Sets source file paths. If not set, source sets to stdin. (default: None)
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
$ ./rhuffle --src in1.txt in2.txt --dst out.txt --head 2
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