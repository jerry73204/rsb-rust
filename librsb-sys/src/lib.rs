//! FFI interface for librsb.
//!
//! # Usage
//!
//! In order to conditionally generate bindings and documents, it is suggested to use the following snipplet in `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! librsb-sys = "X.Y.Z"  # Fill the version here
//!
//! [features]
//! codegen = ["librsb-sys/codegen"]
//! doc-only = ["librsb-sys/doc-only"]
//!
//! [package.metadata.docs.rs]
//! features = ["doc-only"]
//! ```
//!
//! # Cargo Features
//!
//! - `codegen`
//!
//!   Generate FFI bindings from header files.
//!
//! - `doc-only`
//!
//!   Disable binding generation and linking.
//!
//! # Environment Variables
//!
//! The librsb-sys reads the following environment variables.
//!
//! - `LIBRSB_PATH`
//!
//!   The prefix path of the installed librsb library. It defaults to `/usr` if not set.
//!
//! - `LIBRSB_INCLUDE`
//!
//!   The directory to search for C header files. It defaults to `$LIBRSB_PATH/include` if not set.
//!
//! - `LIBRSB_LIBRARY`
//!
//!   The directory to search for library files. It defaults to `$LIBRSB_PATH/lib` if not set.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
pub use bindings::*;
