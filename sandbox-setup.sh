#!/usr/bin/env bash
# allow alias expansion
shopt -s expand_aliases

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

function log() {
  printf "${GREEN}[+] $1${NC}\n"
}

function fail() {
  printf "${RED}[!] $1${NC}\n"
  exit -1
}

TEZOS_DIR=$1 
if [[ $TEZOS_DIR = "" ]]; then
   fail "Missing /full/path/to/tezos repo dir as first arg"
fi

SANDBOX_DIR=./sandbox_data
rm -rf $SANDBOX_DIR
mkdir -p $SANDBOX_DIR

# suppress tezos client warnings
export TEZOS_CLIENT_UNSAFE_DISABLE_DISCLAIMER=Y

# start sandboxed node
log "starting sandboxed node..."
DATA_DIR="$SANDBOX_DIR/tezos/node" $TEZOS_DIR/src/bin_node/tezos-sandboxed-node.sh 1 --connections 1 &
P1=$!

# wait some time for node to start
eval 'sleep 5' &
P2=$!
wait $P2

# initialize sandboxed client
log "initializing sandboxed client..."
mkdir -p $SANDBOX_DIR/tezos/client
cp local-tezos-init-sandboxed-client.sh $TEZOS_DIR/src/bin_client/
eval "$(SANDBOX_TEZOS_TMP_CLIENT=$PWD/sandbox_data/tezos/client $TEZOS_DIR/src/bin_client/local-tezos-init-sandboxed-client.sh 1)"

sleep 5
# activate alpha protocol
log "activate alpha..."
set -x
tezos-activate-alpha
set +x

log "activated chains:"
$TEZOS_DIR/src/bin_client//../../_build/default/src/bin_client/main_client.exe -base-dir $SANDBOX_DIR/tezos/client -endpoint http://localhost:18731 rpc get /chains/main/blocks/head/metadata

log "known addresses:"
$TEZOS_DIR/src/bin_client//../../_build/default/src/bin_client/main_client.exe -base-dir $SANDBOX_DIR/tezos/client -endpoint http://localhost:18731 list known addresses

# bake in a loop every 5 seconds, since sandbox does not come with a baker node
while :
do
    log "baking for bootstrap5 account"
	$TEZOS_DIR/src/bin_client//../../_build/default/src/bin_client/main_client.exe -base-dir $SANDBOX_DIR/tezos/client -endpoint http://localhost:18731 bake for bootstrap5
	sleep 5
done

wait $P1
