# Datalog!
A CLI tool for processing w3c-formatted access logs.

## Usage
```
$ processor --help
processor 1.0
Adrian F. <aef@fastmail.com>

USAGE:
    processor [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --display <display>                                                  [default: cli]
    -h, --high-alert-seconds-per-request <high-alert-seconds-per-request>
            Avg. requests per second to trigger an alert. [default: 10]

    -s, --summary-cadence <summary-cadence>
            Summarize logs for every N seconds of log lines [default: 10]
```

## Example output
```
==== 2021-04-19 02:12:58 UTC | 10s ====
400 17 23%
401 15 20%
403 8 11%
404 4 5%
500 3 4%
/ 18 24%
/user 17 23%
/report 16 21%
```

## Build
```
$ cargo build (--release)
Run a simulator:
$ ./target/debug/generator | ./target/debug/processor
Or pass in an existing file:
$ cat ./data/sample.csv | ./target/debug/processor
```

## Test
```
$ cargo test
```

## Requirements
Tested with rustc 1.51.0

## TODO
* Better error handling -- remove unwraps/panics and bubble up errors
* Use enum for "display" CLI arguments
* Benchmark tests!
* Use Tui for CLI display
* Include additional statistics such as top 3 IP addresses and basic anomaly alerts
* Performance optimizations -- especially with Serde.
* Add release build options
* Listen to clippy :*(
* Better directory structure for processor -- rename it!!
* Better quality and more tests!!!!
