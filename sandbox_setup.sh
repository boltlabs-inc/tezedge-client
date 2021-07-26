#!/bin/bash

TEZOS_DIR=$1 
if [[ $TEZOS_DIR = "" ]]; then
   echo "[!] Missing path to tezos repo dir as first arg"
   exit -1
fi

# Initialize sandbox client
export TEZOS_CLIENT_UNSAFE_DISABLE_DISCLAIMER=Y
eval `$TEZOS_DIR/src/bin_client/tezos-init-sandboxed-client.sh 1`

tezos-activate-alpha

# bake 1000 blocks at 1 second intervals
for i in {1..1000}
        do tezos-client -endpoint http://127.0.0.1:18731 bake for bootstrap5 --minimal-timestamp
        sleep 1
done
