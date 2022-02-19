use crate::{
    common::*,
    error::{check, RSB_ERR_NO_ERROR},
    mtx::Mtx,
    r#type::RsbType,
};

pub struct CsrMatrix<T, const NR: usize, const NC: usize>
where
    T: RsbType,
{
    mtx: Mtx<T>,
}

impl<T, const NR: usize, const NC: usize> CsrMatrix<T, NR, NC> where T: RsbType {}
