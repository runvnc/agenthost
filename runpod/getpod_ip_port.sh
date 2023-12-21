#!/bin/bash
all=$(curl --request POST \
  --header 'content-type: application/json' \
  --silent --url "https://api.runpod.io/graphql?api_key=${RUNPOD_API_KEY}" \
  --data '{"query": "query Pod { pod(input: {podId: \"'"$1"'\"}) { id name runtime { uptimeInSeconds ports { ip isIpPublic privatePort publicPort type } gpus { id gpuUtilPercent memoryUtilPercent } container { cpuPercent memoryPercent } } } }"}')

#echo $all | jq .
public_ip=$(echo $all | jq -r '.data.pod.runtime.ports[] | select(.isIpPublic == true) | .ip' | head -n 1)

# Extract the port where public and private IPs match
port=$(echo $all | jq -r --arg ip "$public_ip" '.data.pod.runtime.ports[] | select(.isIpPublic == true and .ip == $ip) | "\(.publicPort)"' | head -n 1)
echo $public_ip
echo $port
