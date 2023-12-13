#!/bin/bash

outp=$(vastai search offers 'gpu_name=RTX_4090  dph<0.6 inet_down>1000 geolocation=US cuda_vers=12.2')
echo "$outp"
firstid=$(echo "$outp" |  sed -n '2p' | cut -d' ' -f1)
echo "Selected offer $firstid"
echo "Creating instance.."

vastai create instance $firstid --image nvidia/cuda:12.2.2-runtime-ubuntu22.04 --env '-p 3132:3132' --disk 69 --ssh

instanceid=$(vastai show instances | sed -n '2p' | cut -d' ' -f1)
sshurl=$(vastai ssh-url $instanceid)

ssh_host=$(echo $conn_string | sed -e 's/ssh:\/\/[^@]*@//' -e 's/:.*//')
ssh_port=$(echo $conn_string | sed 's/.*://')

scp -P $ssh_port target/release/agenthost $ssh_host:/root/

echo $sshurl

ssh -p $ssh_host root@$ssh_port

