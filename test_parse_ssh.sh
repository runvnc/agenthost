#!/bin/bash

# The SSH connection string
conn_string="ssh://root@ssh4.vast.ai:14899"

# Extracting the host and port
ssh_host=$(echo $conn_string | sed -e 's/ssh:\/\/[^@]*@//' -e 's/:.*//')
ssh_port=$(echo $conn_string | sed 's/.*://')

# Printing the results
echo "SSH Host: $ssh_host"
echo "SSH Port: $ssh_port"
