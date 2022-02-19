use crate::{
    common::*,
    discard_zero::DiscardZero,
    duplicates::Duplicates,
    error::{check, ensure, Error, Result, RSB_ERR_NO_ERROR},
    numerical_type::NumericalType,
    repr, storage,
    symmetry::Symmetry,
    transpose::Transpose,
    utils,
};

#[derive(Debug)]
pub struct Mtx<T>
where
    T: NumericalType,
{
    ptr: Option<NonNull<sys::rsb_mtx_t>>,
    _phantom: PhantomData<T>,
}

impl<T> Mtx<T>
where
    T: NumericalType,
{
    pub fn try_from_coo_slices(
        nr: sys::rsb_coo_idx_t,
        nc: sys::rsb_coo_idx_t,
        va: &[T],
        ia: &[sys::rsb_coo_idx_t],
        ja: &[sys::rsb_coo_idx_t],
        symmetry: Symmetry,
        duplicates: Duplicates,
    ) -> Result<Self> {
        crate::init::init();

        let nnz = va.len();
        let flags = duplicates.code() | symmetry.code();

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
        symmetry: Symmetry,
        duplicates: Duplicates,
    ) -> Result<Self> {
        crate::init::init();

        let nnz = va.len();
        let flags = duplicates.code() | symmetry.code();

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
        symmetry: Symmetry,
        duplicates: Duplicates,
    ) -> Result<Self> {
        crate::init::init();

        let nnz = va.len();
        let flags = duplicates.code() | symmetry.code();

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

    pub fn spmv<'a, A, R, B, O>(
        &self,
        transpose: Transpose,
        in_scale: A,
        rhs: R,
        out_scale: B,
        output: O,
    ) -> Result<()>
    where
        A: Into<Option<T>>,
        B: Into<Option<T>>,
        R: Into<repr::Vector<'a, T>>,
        O: Into<repr::VectorMut<'a, T>>,
        T: 'a,
    {
        let alpha = in_scale.into();
        let alpha_ptr = alpha
            .as_ref()
            .map(|v| v as *const T)
            .unwrap_or_else(|| ptr::null());

        let beta = out_scale.into();
        let beta_ptr = beta
            .as_ref()
            .map(|v| v as *const T)
            .unwrap_or_else(|| ptr::null());

        let rhs = rhs.into();
        let mut output = output.into();

        unsafe {
            let err = sys::rsb_spmv(
                transpose.code(),
                alpha_ptr as *const c_void,
                self.ptr(),
                rhs.to_ptr(),
                rhs.stride(),
                beta_ptr as *const c_void,
                output.to_ptr(),
                output.stride(),
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn spsv<'a, A, R, O>(&self, transpose: Transpose, scale: A, rhs: R, output: O) -> Result<()>
    where
        A: Into<Option<T>>,
        R: Into<repr::Vector<'a, T>>,
        O: Into<repr::VectorMut<'a, T>>,
        T: 'a,
    {
        let scale = scale.into();
        let scale_ptr = scale
            .as_ref()
            .map(|v| v as *const T)
            .unwrap_or_else(|| ptr::null());
        let rhs = rhs.into();
        let mut output = output.into();

        unsafe {
            let err = sys::rsb_spsv(
                transpose.code(),
                scale_ptr as *const c_void,
                self.ptr(),
                rhs.to_ptr(),
                rhs.stride(),
                output.to_ptr(),
                output.stride(),
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn spsm<'a, A, B, R, O>(
        &self,
        transpose: Transpose,
        alpha: A,
        beta: B,
        rhs: R,
        output: O,
    ) -> Result<()>
    where
        A: Into<Option<T>>,
        B: Into<Option<T>>,
        R: Into<repr::Matrix<'a, T>>,
        O: Into<repr::MatrixMut<'a, T>>,
        T: 'a,
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
        let rhs = rhs.into();
        let mut output = output.into();
        ensure!(
            rhs.order() == output.order(),
            "rhs and output matrix major order must be the same"
        );

        unsafe {
            let err = sys::rsb_spsm(
                transpose.code(),
                alpha_ptr as *const c_void,
                self.ptr(),
                rhs.num_vecs(),
                rhs.order().code(),
                beta_ptr as *const c_void,
                rhs.to_ptr(),
                rhs.leading_dimension(),
                output.to_ptr(),
                output.leading_dimension(),
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn sppsp<A, B>(
        &self,
        self_transpose: Transpose,
        alpha: A,
        rhs_transpose: Transpose,
        beta: B,
        rhs: &Self,
    ) -> Result<Self>
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
            let mut errval = RSB_ERR_NO_ERROR;

            let ptr = sys::rsb_sppsp(
                T::TYPE_CODE,
                self_transpose.code(),
                alpha_ptr as *const c_void,
                self.ptr(),
                rhs_transpose.code(),
                beta_ptr as *const c_void,
                rhs.ptr(),
                &mut errval as *mut _,
            );

            check(errval)?;

            Ok(Self::from_raw(ptr))
        }
    }

    pub fn spmsp<A, B>(
        &self,
        self_transpose: Transpose,
        alpha: A,
        rhs_transpose: Transpose,
        beta: B,
        rhs: &Self,
    ) -> Result<Self>
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
            let mut errval = RSB_ERR_NO_ERROR;

            let ptr = sys::rsb_spmsp(
                T::TYPE_CODE,
                self_transpose.code(),
                alpha_ptr as *const c_void,
                self.ptr(),
                rhs_transpose.code(),
                beta_ptr as *const c_void,
                rhs.ptr(),
                &mut errval as *mut _,
            );

            check(errval)?;

            Ok(Self::from_raw(ptr))
        }
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

    pub fn convert_to<U, S>(
        &self,
        scale: S,
        trans: Transpose,
        discard_zeros: bool,
    ) -> Result<Mtx<U>>
    where
        U: NumericalType,
        S: Into<Option<T>>,
    {
        let scale: Option<T> = scale.into();
        let scale_ptr: *const T = scale
            .as_ref()
            .map(|s| s as *const T)
            .unwrap_or_else(|| ptr::null());
        let discard_zeros = DiscardZero::from(discard_zeros);
        let flags = discard_zeros.code();

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

    pub fn extend_by_coo_slices(
        &mut self,
        rows: &[sys::rsb_coo_idx_t],
        cols: &[sys::rsb_coo_idx_t],
        vals: &[T],
        duplicates: Duplicates,
    ) -> Result<()> {
        let nnz = rows.len();
        ensure!(
            cols.len() == nnz && vals.len() == nnz,
            "the length of slices must be equal"
        );
        let flags = duplicates.code();

        unsafe {
            let err = sys::rsb_mtx_set_vals(
                self.ptr_mut(),
                vals.as_ptr() as *const c_void,
                rows.as_ptr(),
                cols.as_ptr(),
                nnz as sys::rsb_nnz_idx_t,
                flags,
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn get_by_coo_slices(
        &self,
        values: &mut [T],
        rows: &[sys::rsb_coo_idx_t],
        cols: &[sys::rsb_coo_idx_t],
    ) -> Result<()> {
        let nnz = rows.len();
        ensure!(
            nnz == cols.len() && nnz == values.len(),
            "the length of COO slices must be equal"
        );

        unsafe {
            let err = sys::rsb_mtx_get_vals(
                self.ptr(),
                values.as_mut_ptr() as *mut c_void,
                rows.as_ptr(),
                cols.as_ptr(),
                nnz as sys::rsb_nnz_idx_t,
                sys::RSB_FLAG_C_INDICES_INTERFACE as sys::rsb_flags_t,
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn get(&self, row: usize, col: usize) -> Result<T> {
        let mut values = [T::zero()];
        let rows = [row as sys::rsb_coo_idx_t];
        let cols = [col as sys::rsb_coo_idx_t];
        self.get_by_coo_slices(&mut values, &rows, &cols)?;
        Ok(values.into_iter().next().unwrap())
    }

    pub fn mul_scalar(&mut self, mut scalar: T) -> Result<()> {
        unsafe {
            let err = sys::rsb_mtx_upd_vals(
                self.ptr_mut(),
                sys::rsb_elopf_t::RSB_ELOPF_MUL,
                &mut scalar as *mut T as *mut c_void,
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn div_scalar(&mut self, mut scalar: T) -> Result<()> {
        unsafe {
            let err = sys::rsb_mtx_upd_vals(
                self.ptr_mut(),
                sys::rsb_elopf_t::RSB_ELOPF_DIV,
                &mut scalar as *mut T as *mut c_void,
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn pow(&mut self, mut pow: T) -> Result<()> {
        unsafe {
            let err = sys::rsb_mtx_upd_vals(
                self.ptr_mut(),
                sys::rsb_elopf_t::RSB_ELOPF_POW,
                &mut pow as *mut T as *mut c_void,
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn neg_inplace(&mut self) -> Result<()> {
        unsafe {
            let err = sys::rsb_mtx_upd_vals(
                self.ptr_mut(),
                sys::rsb_elopf_t::RSB_ELOPF_NEG,
                ptr::null_mut(),
            );
            check(err)?;
        }

        Ok(())
    }

    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let path = utils::try_osstr_to_cstr(path.as_os_str())
            .map_err(|err| Error::custom(format!("{}", err)))?;

        unsafe {
            let err = sys::rsb_file_mtx_save(self.ptr(), path.as_ptr());
            check(err)?;
        }

        Ok(())
    }

    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let path = utils::try_osstr_to_cstr(path.as_os_str())
            .map_err(|err| Error::custom(format!("{}", err)))?;
        let flags = storage::DEFAULT_STORAGE_FLAGS;

        unsafe {
            let mut errval = RSB_ERR_NO_ERROR;
            let ptr =
                sys::rsb_file_mtx_load(path.as_ptr(), flags, T::TYPE_CODE, &mut errval as *mut _);
            check(errval)?;

            Ok(Self::from_raw(ptr))
        }
    }

    fn ptr(&self) -> *const sys::rsb_mtx_t {
        self.ptr.unwrap().as_ptr()
    }

    fn ptr_mut(&self) -> *mut sys::rsb_mtx_t {
        self.ptr.unwrap().as_ptr()
    }
}

impl<T> Clone for Mtx<T>
where
    T: NumericalType,
{
    fn clone(&self) -> Self {
        self.convert_to(None, Default::default(), true).unwrap()
    }
}

impl<T> Extend<(usize, usize, T)> for Mtx<T>
where
    T: NumericalType,
{
    fn extend<I: IntoIterator<Item = (usize, usize, T)>>(&mut self, iter: I) {
        let (rows, cols, vals) = iter
            .into_iter()
            .map(|(row, col, val)| (row as sys::rsb_coo_idx_t, col as sys::rsb_coo_idx_t, val))
            .unzip_n_vec();
        self.extend_by_coo_slices(&rows, &cols, &vals, Default::default())
            .unwrap();
    }
}

impl<T> Drop for Mtx<T>
where
    T: NumericalType,
{
    fn drop(&mut self) {
        if let Some(mut ptr) = self.ptr.take() {
            unsafe {
                sys::rsb_mtx_free(ptr.as_mut());
            }
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Dims {
//     pub rows: usize,
//     pub cols: usize,
//     pub nonzeros: usize,
// }
