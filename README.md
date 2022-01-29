# r rust panic

A test package to check that the panic in Rust gets properly handdled:

1. The destructor (Drop trait) is executed when panic occurs;
2. The error message is converted into R's error message (into the R's strerr stream).

## How to start

1. You need to change the src/rust/Cargo.toml, where must change th local "extendr-api" cargo library path.
2. (Probably needed, terminal) `cd src/rust` then `cargo clean` to force the cargo library to rebuild.
3. (R) Run `rextendr::document()` then `run(0L)` for the non-throwing code, `run(0.0)` fo the throwing code.
4. (Probably needed, R) Run `pkgbuild::clean_dll()` to clean up the existing DLL files. This occationally needed as when you only touched the extendr library, the `rextendr::document()` may not re-generate the compiled files as it thinks all the source files are unchanged.

## Reference

* The discussion on extendr: https://github.com/extendr/extendr/issues/278#issuecomment-1024944619
