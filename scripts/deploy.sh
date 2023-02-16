#!/bin/bash
set -e

source ./variables.sh

cd .. &&
bash build.sh &&
cd scripts

if [ "$1" == "deploy" ]; then
  near deploy $NFT_CONTRACT_ACCOUNT_ID ../res/$NFT_WASM_NAME new '{"owner_id": "'$OWNER_ACCOUNT_ID'", "metadata": {"spec": "spec", "name": "name", "symbol": "symbol"} }'
elif [ "$1" == "redeploy" ]; then
  near deploy $NFT_CONTRACT_ACCOUNT_ID ../res/$NFT_WASM_NAME
elif [ "$1" == "clean" ]; then
  bash clear-state.sh && near deploy $NFT_CONTRACT_ACCOUNT_ID ../res/$NFT_WASM_NAME new '{"owner_id": "'$OWNER_ACCOUNT_ID'", "metadata": {"spec": "spec", "name": "name", "symbol": "symbol"} }'
fi