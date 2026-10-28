#!/usr/bin/env bash
set -euo pipefail

NETWORK="${STELLAR_NETWORK:-testnet}"
IDENTITY="${STELLAR_IDENTITY:-deployer}"
MIN_STAKE="${MIN_STAKE:-1000000000}"       # 100 tokens (7 decimals)
UNBOND_PERIOD="${UNBOND_PERIOD:-86400}"    # 24 hours in seconds
FEE_BPS="${FEE_BPS:-500}"                  # 5.00% protocol fee

echo "==> [1/6] Building Soroban WASM contracts..."
stellar contract build

WASM_DIR="target/wasm32v1-none/release"
if [ ! -d "$WASM_DIR" ]; then
  WASM_DIR="target/wasm32-unknown-unknown/release"
fi

ADMIN_ADDR=$(stellar keys address "$IDENTITY")
TREASURY_ADDR="${TREASURY_ADDRESS:-$ADMIN_ADDR}"

if [ -z "${STAKE_TOKEN_ADDRESS:-}" ]; then
  echo "==> Wrapping native XLM asset on $NETWORK for stake/bounty token..."
  STAKE_TOKEN_ADDRESS=$(stellar contract asset deploy --asset native --source "$IDENTITY" --network "$NETWORK" 2>/dev/null || stellar contract id asset --asset native --network "$NETWORK")
fi
BOUNTY_TOKEN_ADDRESS="${BOUNTY_TOKEN_ADDRESS:-$STAKE_TOKEN_ADDRESS}"

echo "==> [2/6] Deploying keeper-registry..."
KEEPER_REGISTRY_ID=$(stellar contract deploy \
  --wasm "$WASM_DIR/keeper_registry.wasm" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo "==> Initializing keeper-registry ($KEEPER_REGISTRY_ID)..."
stellar contract invoke \
  --id "$KEEPER_REGISTRY_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  initialize \
  --admin "$ADMIN_ADDR" \
  --stake_token "$STAKE_TOKEN_ADDRESS" \
  --min_stake "$MIN_STAKE" \
  --unbond_period "$UNBOND_PERIOD"

echo "==> [3/6] Deploying bounty-vault..."
BOUNTY_VAULT_ID=$(stellar contract deploy \
  --wasm "$WASM_DIR/bounty_vault.wasm" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo "==> Initializing bounty-vault ($BOUNTY_VAULT_ID)..."
stellar contract invoke \
  --id "$BOUNTY_VAULT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  initialize \
  --admin "$ADMIN_ADDR" \
  --token "$BOUNTY_TOKEN_ADDRESS" \
  --fee_bps "$FEE_BPS" \
  --treasury "$TREASURY_ADDR"

echo "==> [4/6] Deploying job-engine..."
JOB_ENGINE_ID=$(stellar contract deploy \
  --wasm "$WASM_DIR/job_engine.wasm" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo "==> Initializing job-engine ($JOB_ENGINE_ID)..."
stellar contract invoke \
  --id "$JOB_ENGINE_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  initialize \
  --admin "$ADMIN_ADDR" \
  --keeper_registry "$KEEPER_REGISTRY_ID" \
  --bounty_vault "$BOUNTY_VAULT_ID"

echo "==> [5/6] Authorizing job-engine in keeper-registry..."
stellar contract invoke \
  --id "$KEEPER_REGISTRY_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  set_engine \
  --engine "$JOB_ENGINE_ID"

echo "==> [6/6] Authorizing job-engine in bounty-vault..."
stellar contract invoke \
  --id "$BOUNTY_VAULT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  set_engine \
  --engine "$JOB_ENGINE_ID"

echo ""
echo "============================================================"
echo "  TAKT PROTOCOL DEPLOYMENT COMPLETE ($NETWORK)"
echo "============================================================"
echo "NEXT_PUBLIC_STELLAR_NETWORK=$NETWORK"
echo "NEXT_PUBLIC_ADMIN_ADDRESS=$ADMIN_ADDR"
echo "NEXT_PUBLIC_TOKEN_ADDRESS=$BOUNTY_TOKEN_ADDRESS"
echo "NEXT_PUBLIC_KEEPER_REGISTRY_ID=$KEEPER_REGISTRY_ID"
echo "NEXT_PUBLIC_BOUNTY_VAULT_ID=$BOUNTY_VAULT_ID"
echo "NEXT_PUBLIC_JOB_ENGINE_ID=$JOB_ENGINE_ID"
echo "============================================================"
