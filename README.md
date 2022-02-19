# librsb for Rust

This project implements FFI interface in Rust for [`librsb`](http://librsb.sourceforge.net/) library. It provides multi-threaded sparse matrix computations using the Recursive Sparse Blocks matrix format. The repository provides two crates: `librsb-sys` and `rsb`, respectively the FFI bindings and Rust wrappers.

## `librsb-sys` Crate

The librsb-sys is available on crates.io. In order to conditionally generate bindings and documents, it is suggested to use the following snipplet in `Cargo.toml`.

```toml
[dependencies]
librsb-sys = "X.Y.Z"  # Fill the version here

[features]
codegen = ["librsb-sys/codegen"]
doc-only = ["librsb-sys/doc-only"]

[package.metadata.docs.rs]
features = ["doc-only"]
```

The librsb-sys reads the following environment variables.

- `LIBRSB_PATH`

  The prefix path of the installed librsb library. It defaults to `/usr` if not set.

- `LIBRSB_INCLUDE`

  The directory to search for C header files. It defaults to `$LIBRSB_PATH/include` if not set.

- `LIBRSB_LIBRARY`

  The directory to search for library files. It defaults to `$LIBRSB_PATH/lib` if not set.

## `rsb` Crate

The `rsb` crate is not available on crates.io yet and is not matured yet. If you want to go bleeding edge, include the git repository in your `Cargo.toml`.

```toml
[dependencies]
rsb = { git = "https://github.com/jerry73204/rsb-rust.git", branch = "master" }
```

## License

LGPL-3.0. See [license file](LICENSE.txt).
