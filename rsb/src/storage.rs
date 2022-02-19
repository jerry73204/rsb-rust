use crate::common::*;

pub(crate) const DEFAULT_STORAGE_FLAGS: sys::rsb_flags_t =
    (sys::RSB_FLAG_WANT_BCSS_STORAGE | sys::RSB_FLAG_WANT_COO_STORAGE) as sys::rsb_flags_t;
