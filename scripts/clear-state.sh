#!/bin/bash
set -e

source ./variables.sh

# reference: https://github.com/near/core-contracts/tree/master/state-cleanup
# 1. deploy state_cleanup wasm
near deploy $NFT_CONTRACT_ACCOUNT_ID ../res/state_cleanup.wasm &&
# 2. cleanup state
python3 state-clean.py $NFT_CONTRACT_ACCOUNT_ID $NFT_ACCOUNT_ID