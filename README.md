# r rust panic

A test package to check that the panic in Rust gets properly handdled:

1. The destructor (Drop trait) is executed when panic occurs;
2. The error message is converted into R's error message (into the R's strerr stream).

## How to start

You need to change the src/rust/Cargo.toml, if you are going to use the local version of "extendr-api" cargo library.

## Reference

* The discussion on extendr: https://github.com/extendr/extendr/issues/278#issuecomment-1024944619
