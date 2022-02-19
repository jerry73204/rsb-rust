use crate::{
    common::*,
    error::{ensure, Result},
    major_order::MajorOrder,
};

pub use matrix::*;
mod matrix {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Matrix<'a, T> {
        pub(super) order: MajorOrder,
        pub(super) hw: [usize; 2],
        pub(super) slice: &'a [T],
    }

    impl<'a, T> Matrix<'a, T> {
        pub fn from_row_slice(nrows: usize, ncols: usize, slice: &'a [T]) -> Result<Self> {
            Self::from_slice(nrows, ncols, MajorOrder::Row, slice)
        }

        pub fn from_col_slice(nrows: usize, ncols: usize, slice: &'a [T]) -> Result<Self> {
            Self::from_slice(nrows, ncols, MajorOrder::Column, slice)
        }

        pub fn from_slice(
            nrows: usize,
            ncols: usize,
            order: MajorOrder,
            slice: &'a [T],
        ) -> Result<Self> {
            ensure!(
                slice.len() == nrows * ncols,
                "the shape (rows, cols) = ({}, {}) does not match the slice length {}",
                nrows,
                ncols,
                slice.len()
            );

            Ok(Self {
                order,
                hw: [nrows, ncols],
                slice,
            })
        }

        pub fn from_array<const N1: usize, const N2: usize>(
            order: MajorOrder,
            array: &'a [[T; N2]; N1],
        ) -> Self {
            let slice = array.flat();
            let hw = match order {
                MajorOrder::Column => [N1, N2],
                MajorOrder::Row => [N2, N1],
            };
            Self { order, hw, slice }
        }

        pub fn from_col_array<const NR: usize, const NC: usize>(array: &'a [[T; NC]; NR]) -> Self {
            Self::from_array(MajorOrder::Column, array)
        }

        pub fn from_row_array<const NR: usize, const NC: usize>(array: &'a [[T; NR]; NC]) -> Self {
            Self::from_array(MajorOrder::Column, array)
        }

        /// Get the matrix repr's order.
        pub fn order(&self) -> MajorOrder {
            self.order
        }

        /// Get the matrix repr's slice.
        pub fn slice(&self) -> &[T] {
            self.slice
        }

        pub(crate) fn to_ptr(&self) -> *const c_void {
            self.slice.as_ptr() as *const c_void
        }

        /// Get the matrix repr's hw.
        pub fn hw(&self) -> [usize; 2] {
            self.hw
        }

        pub(crate) fn leading_dimension(&self) -> sys::rsb_nnz_idx_t {
            let value = match self.order {
                MajorOrder::Column => self.hw[0],
                MajorOrder::Row => self.hw[1],
            };
            value as sys::rsb_nnz_idx_t
        }

        pub(crate) fn num_vecs(&self) -> sys::rsb_coo_idx_t {
            let value = match self.order {
                MajorOrder::Column => self.hw[1],
                MajorOrder::Row => self.hw[0],
            };
            value as sys::rsb_coo_idx_t
        }
    }
}

pub use matrix_mut::*;
mod matrix_mut {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MatrixMut<'a, T> {
        pub(super) order: MajorOrder,
        pub(super) hw: [usize; 2],
        pub(super) slice: &'a mut [T],
    }

    impl<'a, T> MatrixMut<'a, T> {
        pub fn from_row_slice(nrows: usize, ncols: usize, slice: &'a mut [T]) -> Result<Self> {
            Self::from_slice(nrows, ncols, MajorOrder::Row, slice)
        }

        pub fn from_col_slice(nrows: usize, ncols: usize, slice: &'a mut [T]) -> Result<Self> {
            Self::from_slice(nrows, ncols, MajorOrder::Column, slice)
        }

        pub fn from_slice(
            nrows: usize,
            ncols: usize,
            order: MajorOrder,
            slice: &'a mut [T],
        ) -> Result<Self> {
            ensure!(
                slice.len() == nrows * ncols,
                "the shape (rows, cols) = ({}, {}) does not match the slice length {}",
                nrows,
                ncols,
                slice.len()
            );

            Ok(Self {
                order,
                hw: [nrows, ncols],
                slice,
            })
        }

        pub fn from_array<const N1: usize, const N2: usize>(
            order: MajorOrder,
            array: &'a mut [[T; N2]; N1],
        ) -> Self {
            let slice = array.flat_mut();
            let hw = match order {
                MajorOrder::Column => [N1, N2],
                MajorOrder::Row => [N2, N1],
            };
            Self { order, hw, slice }
        }

        pub fn from_col_array<const NR: usize, const NC: usize>(
            array: &'a mut [[T; NC]; NR],
        ) -> Self {
            Self::from_array(MajorOrder::Column, array)
        }

        pub fn from_row_array<const NR: usize, const NC: usize>(
            array: &'a mut [[T; NR]; NC],
        ) -> Self {
            Self::from_array(MajorOrder::Column, array)
        }

        /// Get the matrix repr's order.
        pub fn order(&self) -> MajorOrder {
            self.order
        }

        /// Get the matrix repr's slice.
        pub fn slice(&mut self) -> &mut [T] {
            self.slice
        }

        pub(crate) fn to_ptr(&mut self) -> *mut c_void {
            self.slice.as_mut_ptr() as *mut c_void
        }

        /// Get the matrix repr's hw.
        pub fn hw(&self) -> [usize; 2] {
            self.hw
        }

        pub(crate) fn leading_dimension(&self) -> sys::rsb_nnz_idx_t {
            let value = match self.order {
                MajorOrder::Column => self.hw[0],
                MajorOrder::Row => self.hw[1],
            };
            value as sys::rsb_nnz_idx_t
        }

        pub(crate) fn num_vecs(&self) -> sys::rsb_coo_idx_t {
            let value = match self.order {
                MajorOrder::Column => self.hw[1],
                MajorOrder::Row => self.hw[0],
            };
            value as sys::rsb_coo_idx_t
        }
    }
}

pub use vector::*;
mod vector {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Vector<'a, T> {
        pub(super) stride: usize,
        pub(super) slice: &'a [T],
    }

    impl<'a, T> Vector<'a, T> {
        pub fn from_slice(slice: &'a [T]) -> Self {
            Self { slice, stride: 1 }
        }

        pub fn from_strided_slice(stride: usize, slice: &'a [T]) -> Result<Self> {
            ensure!(stride >= 1, "stride must be positive, but get zero");
            ensure!(
                slice.len() % stride == 0,
                "stride {} is not multiple of slice length {}",
                stride,
                slice.len()
            );
            Ok(Self { slice, stride })
        }

        pub(crate) fn to_ptr(&self) -> *const c_void {
            self.slice.as_ptr() as *const c_void
        }

        /// Get the vector's stride.
        pub(crate) fn stride(&self) -> sys::rsb_coo_idx_t {
            self.stride as sys::rsb_coo_idx_t
        }
    }

    impl<'a, T> From<&'a [T]> for Vector<'a, T> {
        fn from(slice: &'a [T]) -> Self {
            Self::from_slice(slice)
        }
    }
}

pub use vector_mut::*;
mod vector_mut {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VectorMut<'a, T> {
        pub(super) stride: usize,
        pub(super) slice: &'a mut [T],
    }

    impl<'a, T> VectorMut<'a, T> {
        pub fn from_slice(slice: &'a mut [T]) -> Self {
            Self { slice, stride: 1 }
        }

        pub fn from_strided_slice(stride: usize, slice: &'a mut [T]) -> Result<Self> {
            ensure!(stride >= 1, "stride must be positive, but get zero");
            ensure!(
                slice.len() % stride == 0,
                "stride {} is not multiple of slice length {}",
                stride,
                slice.len()
            );
            Ok(Self { slice, stride })
        }

        pub(crate) fn to_ptr(&mut self) -> *mut c_void {
            self.slice.as_mut_ptr() as *mut c_void
        }

        /// Get the vector mut's stride.
        pub(crate) fn stride(&self) -> sys::rsb_coo_idx_t {
            self.stride as sys::rsb_coo_idx_t
        }
    }

    impl<'a, T> From<&'a mut [T]> for VectorMut<'a, T> {
        fn from(slice: &'a mut [T]) -> Self {
            Self::from_slice(slice)
        }
    }
}
