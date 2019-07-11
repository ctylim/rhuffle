# rhuffle

**rhuffle** is a random shuffler for large file with many lines which can exceed available RAM.

## How to Build

```
$ cargo build --release
```

## Usages

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

    -h, --head <NUMBER>                            Sets first `n` lines without shuffling (default: 0).
    -l, --level <hard|soft>                        Sets shuffle level. (default: hard)
        --log <off|error|warn|info|debug|trace>    Sets log level. (default: off)
        --src <Option<PATH>>
            Sets source file path. If not set, source sets to stdin. (default: None)
```

## Benchmarks

TBD