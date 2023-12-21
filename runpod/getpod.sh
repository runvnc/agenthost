#!/bin/bash
echo $1
set -x
curl --request POST \
  --header 'content-type: application/json' \
  --url "https://api.runpod.io/graphql?api_key=${RUNPOD_API_KEY}" \
  --data '{"query": "query Pod { pod(input: {podId: \"'"$1"'\"}) { id name runtime { uptimeInSeconds ports { ip isIpPublic privatePort publicPort type } gpus { id gpuUtilPercent memoryUtilPercent } container { cpuPercent memoryPercent } } } }"}'
