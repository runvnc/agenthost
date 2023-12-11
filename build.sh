#!/bin/bash
export LLAMA_CUDA_DMMV_X=32
export LLAMA_CUDA_DMMV_Y=1
export LLAMA_CUDA_KQUANTS_ITER=2

cargo build --release
