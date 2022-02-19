use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Duplicates {
    Sum,
    Overwrite,
}

impl Duplicates {
    pub fn code(&self) -> sys::rsb_flags_t {
        let flag = match self {
            Self::Sum => sys::RSB_FLAG_DUPLICATES_SUM,
            Self::Overwrite => sys::RSB_FLAG_DUPLICATES_KEEP_LAST,
        };
        flag as sys::rsb_flags_t
    }
}

impl Default for Duplicates {
    fn default() -> Self {
        Self::Overwrite
    }
}
