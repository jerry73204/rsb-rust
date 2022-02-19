pub use itertools::Itertools as _;
pub use librsb_sys as sys;
pub use slice_of_array::prelude::*;
pub use std::{
    borrow::Cow,
    ffi::c_void,
    fmt,
    fmt::Display,
    marker::PhantomData,
    ops::Deref,
    os::raw::{c_char, c_int, c_uint},
    path::Path,
    ptr,
    ptr::NonNull,
    time::Duration,
};

use unzip_n::unzip_n;

unzip_n!(pub 3);
