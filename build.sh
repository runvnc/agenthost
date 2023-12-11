#!/bin/bash
export LLAMA_CUBLAS=1
export CUDA_DOCKER_ARCH=all

cargo build --release
