use crate::error::Result;
use std::ptr::NonNull;

use crate::{
    common::*,
    error::{check, Error, RSB_ERR_NO_ERROR},
    r#type::RsbType,
    transpose::{self, Transpose},
};

#[derive(Debug)]
pub struct Mtx<T>
where
    T: RsbType,
{
    ptr: Option<NonNull<sys::rsb_mtx_t>>,
    _phantom: PhantomData<T>,
}

impl<T> Mtx<T>
where
    T: RsbType,
{
    pub fn try_from_coo_slices(
        nr: sys::rsb_coo_idx_t,
        nc: sys::rsb_coo_idx_t,
        va: &[T],
        ia: &[sys::rsb_coo_idx_t],
        ja: &[sys::rsb_coo_idx_t],
    ) -> Result<Self> {
        crate::init::init();

        let nnz = va.len();
        let flags = sys::RSB_FLAG_NOFLAGS | sys::RSB_FLAG_DUPLICATES_SUM;

        let mtx = unsafe {
            let mut errval = RSB_ERR_NO_ERROR;
            let mtx = sys::rsb_mtx_alloc_from_coo_const(
                va.as_ptr() as *const c_void,
                ia.as_ptr(),
                ja.as_ptr(),
                nnz as sys::rsb_nnz_idx_t,
                T::TYPE_CODE,
                nr,
                nc,
                sys::RSB_DEFAULT_BLOCKING as sys::rsb_blk_idx_t,
                sys::RSB_DEFAULT_BLOCKING as sys::rsb_blk_idx_t,
                flags as sys::rsb_flags_t,
                &mut errval as *mut _,
            );
            check(errval)?;
            mtx
        };

        unsafe { Ok(Mtx::from_raw(mtx)) }
    }

    pub fn try_from_csc_slices(
        nr: sys::rsb_coo_idx_t,
        nc: sys::rsb_coo_idx_t,
        va: &[T],
        ia: &[sys::rsb_coo_idx_t],
        cp: &[sys::rsb_coo_idx_t],
    ) -> Result<Self> {
        crate::init::init();

        let nnz = va.len();
        let flags = sys::RSB_FLAG_NOFLAGS | sys::RSB_FLAG_DUPLICATES_SUM;

        let mtx = unsafe {
            let mut errval = RSB_ERR_NO_ERROR;
            let mtx = sys::rsb_mtx_alloc_from_csc_const(
                va.as_ptr() as *const c_void,
                ia.as_ptr(),
                cp.as_ptr(),
                nnz as sys::rsb_nnz_idx_t,
                T::TYPE_CODE,
                nr,
                nc,
                sys::RSB_DEFAULT_BLOCKING as sys::rsb_blk_idx_t,
                sys::RSB_DEFAULT_BLOCKING as sys::rsb_blk_idx_t,
                flags as sys::rsb_flags_t,
                &mut errval as *mut _,
            );
            check(errval)?;
            mtx
        };

        unsafe { Ok(Mtx::from_raw(mtx)) }
    }

    pub fn try_from_csr_slices(
        nr: sys::rsb_coo_idx_t,
        nc: sys::rsb_coo_idx_t,
        va: &[T],
        rp: &[sys::rsb_coo_idx_t],
        ja: &[sys::rsb_coo_idx_t],
    ) -> Result<Self> {
        crate::init::init();

        let nnz = va.len();
        let flags = sys::RSB_FLAG_NOFLAGS | sys::RSB_FLAG_DUPLICATES_SUM;

        let mtx = unsafe {
            let mut errval = RSB_ERR_NO_ERROR;
            let mtx = sys::rsb_mtx_alloc_from_csr_const(
                va.as_ptr() as *const c_void,
                rp.as_ptr(),
                ja.as_ptr(),
                nnz as sys::rsb_nnz_idx_t,
                T::TYPE_CODE,
                nr,
                nc,
                sys::RSB_DEFAULT_BLOCKING as sys::rsb_blk_idx_t,
                sys::RSB_DEFAULT_BLOCKING as sys::rsb_blk_idx_t,
                flags as sys::rsb_flags_t,
                &mut errval as *mut _,
            );
            check(errval)?;
            mtx
        };

        unsafe { Ok(Mtx::from_raw(mtx)) }
    }

    pub fn spmv<A, B>(
        &self,
        x_vec: &[T],
        y_vec: &mut [T],
        alpha: A,
        beta: B,
        transpose: Transpose,
    ) -> Result<()>
    where
        A: Into<Option<T>>,
        B: Into<Option<T>>,
    {
        let alpha = alpha.into();
        let alpha_ptr = alpha
            .as_ref()
            .map(|v| v as *const T)
            .unwrap_or_else(|| ptr::null());

        let beta = beta.into();
        let beta_ptr = beta
            .as_ref()
            .map(|v| v as *const T)
            .unwrap_or_else(|| ptr::null());

        unsafe {
            let err = sys::rsb_spmv(
                transpose.code(),
                alpha_ptr as *const c_void,
                self.ptr(),
                x_vec.as_ptr() as *mut c_void,
                1,
                beta_ptr as *const c_void,
                y_vec.as_mut_ptr() as *mut c_void,
                1,
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn spsv<A>(
        &self,
        x_vec: &[T],
        y_vec: &mut [T],
        alpha: A,
        transpose: Transpose,
    ) -> Result<()>
    where
        A: Into<Option<T>>,
    {
        let alpha = alpha.into();
        let alpha_ptr = alpha
            .as_ref()
            .map(|v| v as *const T)
            .unwrap_or_else(|| ptr::null());

        unsafe {
            let err = sys::rsb_spsv(
                transpose.code(),
                alpha_ptr as *const c_void,
                self.ptr(),
                x_vec.as_ptr() as *mut c_void,
                1,
                y_vec.as_mut_ptr() as *mut c_void,
                1,
            );
            check(err)?;
        }

        Ok(())
    }

    pub unsafe fn from_raw(ptr: *mut sys::rsb_mtx_t) -> Self {
        Self {
            ptr: Some(NonNull::new(ptr).unwrap()),
            _phantom: PhantomData,
        }
    }

    pub fn into_raw(mut self) -> *mut sys::rsb_mtx_t {
        unsafe { self.ptr.take().unwrap().as_mut() }
    }

    pub fn convert_to<U, S>(&self, scale: S, trans: Transpose) -> Result<Mtx<U>>
    where
        U: RsbType,
        S: Into<Option<T>>,
    {
        let scale: Option<T> = scale.into();
        let scale_ptr: *const T = scale
            .as_ref()
            .map(|s| s as *const T)
            .unwrap_or_else(|| ptr::null());
        let flags = sys::RSB_FLAG_IDENTICAL_FLAGS;

        let ptr = unsafe {
            let mut ptr: *mut sys::rsb_mtx_t = ptr::null_mut();
            let err = sys::rsb_mtx_clone(
                &mut ptr as *mut _,
                U::TYPE_CODE,
                trans.code(),
                scale_ptr as *const c_void,
                self.ptr(),
                flags as sys::rsb_flags_t,
            );
            check(err)?;
            ptr
        };

        unsafe { Ok(Mtx::from_raw(ptr)) }
    }

    fn ptr(&self) -> *const sys::rsb_mtx_t {
        self.ptr.unwrap().as_ptr()
    }
}

impl<T> Clone for Mtx<T>
where
    T: RsbType,
{
    fn clone(&self) -> Self {
        self.convert_to(None, Default::default()).unwrap()
    }
}

impl<T> Drop for Mtx<T>
where
    T: RsbType,
{
    fn drop(&mut self) {
        if let Some(mut ptr) = self.ptr.take() {
            unsafe {
                sys::rsb_mtx_free(ptr.as_mut());
            }
        }
    }
}
