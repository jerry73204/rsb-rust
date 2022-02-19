# librsb-sys

The librsb-sys provides FFI bindings to [librsb](librsb.sourceforge.net/) library.
In order to conditionally generate bindings and documents,
it is suggested to use the following snipplet in `Cargo.toml`.

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

## License

LGPL-3.0. See [license file](LICENSE.txt)
