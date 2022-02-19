use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorOrder {
    Column,
    Row,
}

impl MajorOrder {
    pub fn code(&self) -> sys::rsb_flags_t {
        match self {
            MajorOrder::Column => sys::RSB_FLAG_WANT_COLUMN_MAJOR_ORDER as sys::rsb_flags_t,
            MajorOrder::Row => sys::RSB_FLAG_WANT_ROW_MAJOR_ORDER as sys::rsb_flags_t,
        }
    }
}
