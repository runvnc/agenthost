#!/bin/bash

start_time=$(date +%s)

while true; do
    if curl https://r69krutxgj2l8i-3133.proxy.runpod.net/k; then
        end_time=$(date +%s)
        elapsed_time=$((end_time - start_time))
        echo "Time taken: $elapsed_time seconds"
        break
    else
        sleep 1
    fi
done
