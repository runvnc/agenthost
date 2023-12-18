#!/bin/bash
export LLAMA_CUBLAS=1
export CUDA_DOCKER_ARCH=all

cargo build --release

mkdir dist
cp target/release/agenthost dist

cp -r static dist/
cp -r scripts dist/

mkdir dist/models
mkdir dist/sessions
mkdir dist/sandbox

tar -czvf releases/agenthost.tar.gz dist

