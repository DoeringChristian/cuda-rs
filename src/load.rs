use std::env;
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
fn find_cuda_lib_dirs() -> PathBuf {
    todo!()
}
#[cfg(not(target_os = "windows"))]
fn find_cuda_lib_dirs() -> PathBuf {
    let globs = ["/usr/lib64/libcuda.so.*", "/usr/lib/libcuda.so"];

    let paths = globs
        .iter()
        .map(|pat| glob::glob(pat).unwrap().map(|g| g.unwrap()))
        .flatten()
        .collect::<Vec<_>>();

    if paths.len() == 1 {
        paths[0].clone()
    } else if paths.len() > 1 {
        paths[0].clone()
    } else {
        panic!("libcuda.so not found!")
    }
}

impl crate::cuda::CudaApi {
    pub unsafe fn find_and_load() -> Result<Self, libloading::Error> {
        let path = find_cuda_lib_dirs();
        unsafe { Self::new(path) }
    }
}
