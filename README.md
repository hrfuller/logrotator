## Description

This is a CLI utility which writes a long (or infinite) steam of STDIN to
a fixed number of files in a ring buffer fashion. Once all the files have been
filled to their max size, the oldest log file is overwritten first. This can be useful
for logging in a disk space constrained environment.


## Installation
* Install a rust toolchain with [rustup](https://rustup.rs/)
* `cargo build --release` to build an optimized version for the current platform.


## TODO
* tests.
* Read other streams besides STDIN
* Accept a formattable filename template from the user.
* Write log files in python RotatingFileHandler fashion, so the log file names
  always indicate the time series of the log file.
