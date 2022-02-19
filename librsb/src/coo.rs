use crate::{
    common::*,
    error::{check, RSB_ERR_NO_ERROR},
    mtx::Mtx,
    r#type::RsbType,
};

#[derive(Debug, Clone)]
pub struct CooMatrix<T, const NR: usize, const NC: usize>
where
    T: RsbType,
{
    mtx: Mtx<T>,
}

impl<T, const NR: usize, const NC: usize> CooMatrix<T, NR, NC> where T: RsbType {}
