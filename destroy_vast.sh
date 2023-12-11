#!/bin/bash
id=$(vastai show instances | sed -n '2p' | cut -d' ' -f1)
vastai destroy $id

