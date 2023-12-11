#!/bin/bash

outp=$(vastai search offers 'gpu_name=RTX_4090  dph<0.6 inet_down>1000 geolocation=US cuda_vers=12.2')
firstid=$(echo "$outp" |  sed -n '2p' | cut -d' ' -f1)

vastai create instance $firstid --image nvidia/cuda:12.2.2-runtime-ubuntu22.04 --env 'null' --disk 40 --ssh
vastai show instances
