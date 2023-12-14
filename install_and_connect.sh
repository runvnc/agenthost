#!/bin/bash

ssh_host=$1.vast.ai
ssh_port=$2 

echo "Sending agenthost.."
scp -P $ssh_port releases/agenthost.tar.gz root@$ssh_host:/root/
ssh -p $ssh_port root@$ssh_host "tar -xzvf /root/agenthost.tar.gz -C /root/"
#scp -P $ssh_port target/debug/agenthost root@$ssh_host:/root/

echo "Trying to connect.."
ssh -p $ssh_port root@$ssh_host

