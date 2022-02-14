use crate::{
    data_type::DataType,
    error::{CudnnError, IntoResult},
    sys, TensorFormat,
};
use std::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
};

/// A generic description of an n-dimensional dataset.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TensorDescriptor<T>
where
    T: DataType,
{
    pub(crate) raw: sys::cudnnTensorDescriptor_t,
    data_type: PhantomData<T>,
}

impl<T> TensorDescriptor<T>
where
    T: DataType,
{
    /// Creates a tensor descriptor builder with the given shape and strides.
    ///
    /// # Arguments
    ///
    /// * `shape` - slice containing the size of the tensor for every dimension.
    ///
    /// * `strides` - strides for the tensor descriptor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use cudnn::TensorDescriptor;
    ///
    /// let shape = &[5, 5, 10, 25];
    /// let strides = &[1250, 250, 25, 1];
    ///
    /// let builder = TensorDescriptor::<f32>::new_strides(shape, strides)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_strides(shape: &[i32], strides: &[i32]) -> Result<Self, CudnnError> {
        let mut raw = MaybeUninit::uninit();

        let ndims = shape.len();

        assert_eq!(
            ndims,
            strides.len(),
            "shape and strides length do not match."
        );

        unsafe {
            sys::cudnnCreateTensorDescriptor(raw.as_mut_ptr()).into_result()?;
            let raw = raw.assume_init();

            sys::cudnnSetTensorNdDescriptor(
                raw,
                T::into_raw(),
                ndims as i32,
                shape.as_ptr(),
                strides.as_ptr(),
            )
            .into_result()?;

            Ok(Self {
                raw,
                data_type: PhantomData,
            })
        }
    }

    /// Creates a tensor descriptor builder with the given shape and format.
    ///
    /// # Arguments
    ///
    /// * `shape` - slice containing the size of the tensor for every dimension.
    ///
    /// * `format` - format for the tensor descriptor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use cudnn::{TensorDescriptor, TensorFormat};
    ///
    /// let shape = &[5, 5, 10, 25];
    /// let format = TensorFormat::Nchw;
    ///
    /// let builder = TensorDescriptor::<f32>::new_format(shape, format)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_format(shape: &[i32], format: TensorFormat) -> Result<Self, CudnnError> {
        let mut raw = MaybeUninit::uninit();

        let ndims = shape.len();

        unsafe {
            sys::cudnnCreateTensorDescriptor(raw.as_mut_ptr()).into_result()?;
            let raw = raw.assume_init();

            sys::cudnnSetTensorNdDescriptorEx(
                raw,
                format.into(),
                T::into_raw(),
                ndims as i32,
                shape.as_ptr(),
            )
            .into_result()?;

            Ok(TensorDescriptor {
                raw,
                data_type: PhantomData,
            })
        }
    }
}

impl<T> Drop for TensorDescriptor<T>
where
    T: DataType,
{
    fn drop(&mut self) {
        unsafe {
            sys::cudnnDestroyTensorDescriptor(self.raw);
        }
    }
}
