use crate::common::*;

const DOUBLE: sys::rsb_type_t = sys::rsb_type_t(sys::RSB_NUMERICAL_TYPE_DOUBLE as c_char);
const FLOAT: sys::rsb_type_t = sys::rsb_type_t(sys::RSB_NUMERICAL_TYPE_FLOAT as c_char);
const DOUBLE_COMPLEX: sys::rsb_type_t =
    sys::rsb_type_t(sys::RSB_NUMERICAL_TYPE_DOUBLE_COMPLEX as c_char);
const FLOAT_COMPLEX: sys::rsb_type_t =
    sys::rsb_type_t(sys::RSB_NUMERICAL_TYPE_FLOAT_COMPLEX as c_char);

pub trait NumericalType {
    const TYPE_CODE: sys::rsb_type_t;

    fn zero() -> Self;
}

impl NumericalType for f64 {
    const TYPE_CODE: sys::rsb_type_t = DOUBLE;

    fn zero() -> Self {
        0.0
    }
}

impl NumericalType for f32 {
    const TYPE_CODE: sys::rsb_type_t = FLOAT;

    fn zero() -> Self {
        0.0
    }
}
