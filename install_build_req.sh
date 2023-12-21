#!/bin/bash
apt install libsssl-dev pkg-config 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

