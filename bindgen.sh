#!/bin/bash
set -exu

bindgen \
  --allowlist-type="^CU.*" \
  --allowlist-type="^cuuint(32|64)_t" \
  --allowlist-type="^cudaError_enum" \
  --allowlist-type="^cu.*Complex$" \
  --allowlist-type="^cuda.*" \
  --allowlist-type="^libraryPropertyType.*" \
  --allowlist-var="^CU.*" \
  --allowlist-function="^cu.*" \
  --default-enum-style=rust \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --dynamic-loading CudaApi\
  wrapper.h -- -I/opt/cuda/include \
  > src/cuda.rs
