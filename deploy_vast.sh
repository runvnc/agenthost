#!/bin/bash

outp=$(vastai search offers 'gpu_name=RTX_4090  dph<0.6 inet_down>1000 geolocation=US cuda_vers=12.2')
echo "$outp"
firstid=$(echo "$outp" |  sed -n '2p' | cut -d' ' -f1)
echo "Selected offer $firstid"
echo "Creating instance.."

setip='apt install -y iproute2; export AGENTHOST_HOST=$(ip addr | grep "inet 172" | cut -d"/" -f 1 | cut -d" " -f 6);./agenthost'

created=$(vastai create instance $firstid --image nvidia/cuda:12.2.2-runtime-ubuntu22.04 --env '-p 3132:3132' --disk 69 --ssh --onstart-cmd="$setip")

instanceid=$(vastai show instances | sed -n '2p' | cut -d' ' -f1)
sshurl=$(vastai ssh-url $instanceid)
echo sshurl = "$sshurl"

ssh_host=$(echo "$sshurl" | sed -e 's/ssh:\/\/[^@]*@//' -e 's/:.*//')
ssh_port=$(echo "$sshurl" | sed 's/.*://')

echo "SSH port: $ssh_port"
echo "SSH host: $ssh_host"

