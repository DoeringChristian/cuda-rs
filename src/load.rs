use std::env;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
pub fn find_cuda_lib_dirs() -> Vec<PathBuf> {
    if let Some(root_path) = find_cuda_root() {
        // To do this the right way, we check to see which target we're building for.
        let target = env::var("TARGET")
            .expect("cargo did not set the TARGET environment variable as required.");

        // Targets use '-' separators. e.g. x86_64-pc-windows-msvc
        let target_components: Vec<_> = target.as_str().split('-').collect();

        // We check that we're building for Windows. This code assumes that the layout in
        // CUDA_PATH matches Windows.
        if target_components[2] != "windows" {
            panic!(
                "The CUDA_PATH variable is only used by cuda-sys on Windows. Your target is {}.",
                target
            );
        }

        // Sanity check that the second component of 'target' is "pc"
        debug_assert_eq!(
            "pc", target_components[1],
            "Expected a Windows target to have the second component be 'pc'. Target: {}",
            target
        );

        // x86_64 should use the libs in the "lib/x64" directory. If we ever support i686 (which
        // does not ship with cublas support), its libraries are in "lib/Win32".
        let lib_path = match target_components[0] {
            "x86_64" => "x64",
            "i686" => {
                // lib path would be "Win32" if we support i686. "cublas" is not present in the
                // 32-bit install.
                panic!("Rust cuda-sys does not currently support 32-bit Windows.");
            }
            _ => {
                panic!("Rust cuda-sys only supports the x86_64 Windows architecture.");
            }
        };

        let lib_dir = root_path.join("lib").join(lib_path);

        return if lib_dir.is_dir() {
            vec![lib_dir]
        } else {
            vec![]
        };
    }

    vec![]
}

pub fn read_env() -> Vec<PathBuf> {
    if let Ok(path) = env::var("CUDA_LIBRARY_PATH") {
        // The location of the libcuda, libcudart, and libcublas can be hardcoded with the
        // CUDA_LIBRARY_PATH environment variable.
        let split_char = if cfg!(target_os = "windows") {
            ";"
        } else {
            ":"
        };
        path.split(split_char).map(PathBuf::from).collect()
    } else {
        vec![]
    }
}

#[cfg(not(target_os = "windows"))]
pub fn find_cuda_lib_dirs() -> Vec<PathBuf> {
    let mut candidates = read_env();
    candidates.push(PathBuf::from("/opt/cuda"));
    candidates.push(PathBuf::from("/usr/local/cuda"));
    for e in glob::glob("/usr/local/cuda-*").unwrap().flatten() {
        candidates.push(e)
    }
    candidates.push(PathBuf::from("/usr/lib/cuda"));

    let mut valid_paths = vec![];
    for base in &candidates {
        let lib = PathBuf::from(base).join("lib64");
        if lib.is_dir() {
            valid_paths.push(lib.clone());
            valid_paths.push(lib.join("stubs"));
        }
        let base = base.join("targets/x86_64-linux");
        let header = base.join("include/cuda.h");
        if header.is_file() {
            valid_paths.push(base.join("lib"));
            valid_paths.push(base.join("lib/stubs"));
            continue;
        }
    }
    valid_paths
}
impl crate::cuda::CudaApi {
    pub unsafe fn find_and_load() -> Result<Self, libloading::Error> {
        let paths = find_cuda_lib_dirs();
        let libcuda = paths
            .iter()
            .find(|path| path.join("libcuda.so").exists())
            .unwrap();
        unsafe { Self::new(libcuda) }
    }
}
