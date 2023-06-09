use std::fmt::Display;

use crate::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct CudaError(cudaError_enum);

impl Display for CudaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CUDA error: {:?}({})", self.0, self.0 as u32)
    }
}

impl cudaError_enum {
    pub fn check(self) -> Result<(), CudaError> {
        if self == cudaError_enum::CUDA_SUCCESS {
            Ok(())
        } else {
            Err(CudaError(self))
        }
    }
}
