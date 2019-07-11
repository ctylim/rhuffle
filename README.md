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
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --buf <NUMBER>
            Sets buffer size which is approximately equivalent to available RAM with bytes (default: 3000).

        --dst <PATH>                               Sets destination file path.
    -l, --level <hard|soft>                        Sets shuffle level. (default: hard)
        --log <off|error|warn|info|debug|trace>    Sets log level. (default: off)
        --src <PATH>                               Sets source file path.
```

## Benchmarks

TBD