use crate::common::*;

const RSB_TYPE_INDEX_DOUBLE: sys::rsb_type_t = sys::rsb_type_t(0);
const RSB_TYPE_INDEX_FLOAT: sys::rsb_type_t = sys::rsb_type_t(1);
const RSB_TYPE_INDEX_FLOAT_COMPLEX: sys::rsb_type_t = sys::rsb_type_t(2);
const RSB_TYPE_INDEX_DOUBLE_COMPLEX: sys::rsb_type_t = sys::rsb_type_t(3);

pub trait RsbType {
    const TYPE_CODE: sys::rsb_type_t;
}

impl RsbType for f64 {
    const TYPE_CODE: sys::rsb_type_t = RSB_TYPE_INDEX_DOUBLE;
}

impl RsbType for f32 {
    const TYPE_CODE: sys::rsb_type_t = RSB_TYPE_INDEX_FLOAT;
}
