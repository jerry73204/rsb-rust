use crate::common::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Transpose {
    None,
    Transpose,
    ConjugateTranspose,
}

impl Default for Transpose {
    fn default() -> Self {
        Self::None
    }
}

impl Transpose {
    pub fn code(&self) -> sys::rsb_trans_t {
        match self {
            Self::None => sys::rsb_trans_t(sys::RSB_TRANSPOSITION_N as c_int),
            Self::Transpose => sys::rsb_trans_t(sys::RSB_TRANSPOSITION_T as c_int),
            Self::ConjugateTranspose => sys::rsb_trans_t(sys::RSB_TRANSPOSITION_C as c_int),
        }
    }
}
