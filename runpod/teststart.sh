#!/bin/bash

start_time=$(date +%s)

while true; do
    response=$(curl -s https://r69krutxgj2l8i-3132.proxy.runpod.net/)
    echo $response
    if [[ $response == *"Agent Chat"* ]]; then
        end_time=$(date +%s)
        elapsed_time=$((end_time - start_time))
        echo "Time taken: $elapsed_time seconds"
        break
    else
        echo "Server not responding."
        sleep 1
    fi
done

