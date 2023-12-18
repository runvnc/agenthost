#!/bin/bash
id=$(vastai show instances | sed -n '2p' | cut -d' ' -f1)
echo Destroying $id
vastai destroy instance $id

