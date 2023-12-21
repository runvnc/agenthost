#!/bin/bash

start_time=$(date +%s)

podid=$1

echo "Starting pod $podid.."

runpodctl start pod $podid

while true; do
    sleep 1
    host_port=$(./getpod_ip_port.sh $podid)
    echo "Read $host_port"

    response=$(curl -s http://$host_port/)
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

