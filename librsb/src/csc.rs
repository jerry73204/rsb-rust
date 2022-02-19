use crate::{
    common::*,
    error::{check, RSB_ERR_NO_ERROR},
    mtx::Mtx,
    r#type::RsbType,
};

pub struct CscMatrix<T, const NR: usize, const NC: usize>
where
    T: RsbType,
{
    mtx: Mtx<T>,
}

impl<T, const NR: usize, const NC: usize> CscMatrix<T, NR, NC> where T: RsbType {}
