use crate::common::*;

pub use osstr_to_cstr::*;

mod osstr_to_cstr {
    #![allow(unused_imports)]

    use super::*;
    use cfg_if::cfg_if;
    use error::Error;
    use std::{
        borrow::Cow,
        ffi::{CStr, CString, FromBytesWithNulError, FromVecWithNulError, OsStr},
        fmt,
        fmt::Display,
    };

    /// Convert an [OsStr](std::ffi::OsStr) or an [OsString](std::ffi::OsString) to `Cow<'_, CStr>`.
    ///
    /// The function returns error if the string contains non-terminating null bytes or is not null-terminated.
    pub fn try_osstr_to_cstr<'a, S>(osstr: S) -> Result<Cow<'a, CStr>, Error>
    where
        S: Into<Cow<'a, OsStr>>,
    {
        let osstr = osstr.into();

        Ok({
            cfg_if! {
                if #[cfg(target_family = "unix")] {
                    unix(osstr)?.into()
                } else if #[cfg(target_family = "windows")] {
                    windows(osstr)?.into()
                } else {
                    compile_error!("unsupported target");
                    unreachable!();
                }
            }
        })
    }

    // /// A wrapper to [try_osstr_to_cstr()] that panics on error.
    // ///
    // /// # Panics
    // /// The function panics if the string contains non-terminating null bytes or is not null-terminated.
    // pub fn osstr_to_cstr<'a, S>(osstr: S) -> Cow<'a, CStr>
    // where
    //     S: Into<Cow<'a, OsStr>>,
    // {
    //     try_osstr_to_cstr(osstr).unwrap()
    // }

    #[cfg(target_family = "unix")]
    fn unix(osstr: Cow<'_, OsStr>) -> Result<CString, Error> {
        use std::os::unix::ffi::{OsStrExt as _, OsStringExt as _};

        Ok(match osstr {
            Cow::Borrowed(osstr) => {
                let mut vec = osstr.as_bytes().to_vec();
                vec.push(0);
                CString::from_vec_with_nul(vec)?
            }
            Cow::Owned(osstr) => {
                let mut vec = osstr.into_vec();
                vec.push(0);
                CString::from_vec_with_nul(vec)?
            }
        })
    }

    #[cfg(target_family = "windows")]
    fn windows(osstr: Cow<'_, OsStr>) -> Result<CString, Error> {
        use ascii::{AsciiChar, AsciiString};
        use std::os::windows::ffi::OsStrExt as _;

        let mut ascii: AsciiString = osstr
            .encode_wide()
            .map(AsciiChar::from_ascii)
            .try_collect()?;
        ascii.push(AsciiChar::Null);
        let cstr = CString::from_vec_with_nul(ascii.into())?;
        Ok(cstr)
    }

    pub mod error {
        use super::*;

        #[derive(Debug)]
        pub enum Error {
            FromBytesWithNulError(FromBytesWithNulError),
            FromVecWithNulError(FromVecWithNulError),
            #[cfg(target_family = "windows")]
            ToAsciiCharError(ascii::ToAsciiCharError),
        }

        impl Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Error::FromBytesWithNulError(err) => err.fmt(f),
                    Error::FromVecWithNulError(err) => err.fmt(f),
                    #[cfg(target_family = "windows")]
                    Error::ToAsciiCharError(err) => err.fmt(f),
                }
            }
        }

        impl std::error::Error for Error {}

        #[cfg(target_family = "windows")]
        impl From<ascii::ToAsciiCharError> for Error {
            fn from(v: ascii::ToAsciiCharError) -> Self {
                Self::ToAsciiCharError(v)
            }
        }

        impl From<FromVecWithNulError> for Error {
            fn from(v: FromVecWithNulError) -> Self {
                Self::FromVecWithNulError(v)
            }
        }

        impl From<FromBytesWithNulError> for Error {
            fn from(v: FromBytesWithNulError) -> Self {
                Self::FromBytesWithNulError(v)
            }
        }
    }
}
