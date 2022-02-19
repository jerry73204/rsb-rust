use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DiscardZero(bool);

impl DiscardZero {
    pub fn code(&self) -> sys::rsb_flags_t {
        let flag = if self.0 {
            sys::RSB_FLAG_DISCARD_ZEROS
        } else {
            sys::RSB_FLAG_NOFLAGS
        };
        flag as sys::rsb_flags_t
    }
}

impl From<bool> for DiscardZero {
    fn from(yes: bool) -> Self {
        Self(yes)
    }
}

impl Deref for DiscardZero {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
