#!/bin/bash
set -x
env
echo $AGENTHOST_PHONE_HOME
curl -s "$AGENTHOST_PHONE_HOME/hi?id=$RUNPOD_POD_ID&ip=$RUNPOD_PUBLIC_IP&port=$RUNPOD_TCP_PORT_73132"