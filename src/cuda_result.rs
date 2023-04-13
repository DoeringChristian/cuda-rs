use std::fmt::Display;

use crate::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CudaError {
    CudaError(cudaError_enum),
    LoadingError(libloading::Error),
}

impl Display for CudaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CudaError(err) => {
                write!(f, "CUDA error: {:?} with code {}", err, *err as u32)
            }
            Self::LoadingError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl From<libloading::Error> for CudaError {
    fn from(value: libloading::Error) -> Self {
        CudaError::LoadingError(value)
    }
}

impl cudaError_enum {
    pub fn check(self) -> Result<(), CudaError> {
        if self == cudaError_enum::CUDA_SUCCESS {
            Ok(())
        } else {
            Err(CudaError::CudaError(self))
        }
    }
}
