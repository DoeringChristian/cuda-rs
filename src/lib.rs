#![allow(warnings)]
#![feature(try_trait_v2)]

mod cuda;
mod cuda_result;
mod load;
pub use cuda::*;
pub use cuda_result::*;
