use crate::common::*;

pub trait SymmetryType {
    const SYMMETRY_CODE: sys::rsb_flags_t;
}

macro_rules! declare_marker {
    ($name:ident, $code:expr) => {
        pub struct $name {
            _private: [u8; 0],
        }

        impl SymmetryType for $name {
            const SYMMETRY_CODE: sys::rsb_flags_t = $code;
        }
    };
}

declare_marker!(General, 0x0);
declare_marker!(
    Symmetric,
    sys::RSB_FLAG_SYMMETRIC as sys::rsb_flags_t as sys::rsb_flags_t
);
declare_marker!(Hermitian, sys::RSB_FLAG_HERMITIAN as sys::rsb_flags_t);
declare_marker!(Triangular, sys::RSB_FLAG_TRIANGULAR as sys::rsb_flags_t);
declare_marker!(
    LowerSymmetric,
    sys::RSB_FLAG_LOWER_SYMMETRIC as sys::rsb_flags_t
);
declare_marker!(
    UpperSymmetric,
    sys::RSB_FLAG_UPPER_SYMMETRIC as sys::rsb_flags_t
);
declare_marker!(
    LowerHermitian,
    sys::RSB_FLAG_LOWER_HERMITIAN as sys::rsb_flags_t
);
declare_marker!(
    UpperHermitian,
    sys::RSB_FLAG_UPPER_HERMITIAN as sys::rsb_flags_t
);
declare_marker!(
    LowerTriangular,
    sys::RSB_FLAG_LOWER_TRIANGULAR as sys::rsb_flags_t
);
declare_marker!(
    UpperTriangular,
    sys::RSB_FLAG_UPPER_TRIANGULAR as sys::rsb_flags_t
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symmetry {
    General,
    Symmetric,
    LowerSymmetric,
    UpperSymmetric,
    LowerHermitian,
    UpperHermitian,
    LowerTriangular,
    UpperTriangular,
}

impl Symmetry {
    pub fn code(&self) -> sys::rsb_flags_t {
        let flag = match self {
            Symmetry::General => 0x00,
            Symmetry::Symmetric => sys::RSB_FLAG_SYMMETRIC,
            Symmetry::LowerSymmetric => sys::RSB_FLAG_LOWER_SYMMETRIC,
            Symmetry::UpperSymmetric => sys::RSB_FLAG_UPPER_SYMMETRIC,
            Symmetry::LowerHermitian => sys::RSB_FLAG_LOWER_HERMITIAN,
            Symmetry::UpperHermitian => sys::RSB_FLAG_UPPER_HERMITIAN,
            Symmetry::LowerTriangular => sys::RSB_FLAG_LOWER_TRIANGULAR,
            Symmetry::UpperTriangular => sys::RSB_FLAG_UPPER_TRIANGULAR,
        };
        flag as sys::rsb_flags_t
    }
}
