use crate::common::*;

pub const RSB_ERR_NO_ERROR: sys::rsb_err_t = sys::rsb_err_t(0);

pub(crate) fn check(err: sys::rsb_err_t) -> Result<(), Error> {
    if err == RSB_ERR_NO_ERROR {
        Ok(())
    } else {
        Err(Error::Rsb(err))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Rsb(sys::rsb_err_t),
    Custom(Cow<'static, str>),
}

impl Error {
    pub fn custom<S>(desc: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::Custom(desc.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rsb(code) => writeln!(f, "rsb error: code = {}", code.0),
            Self::Custom(desc) => writeln!(f, "{}", desc),
        }
    }
}

impl std::error::Error for Error {}
