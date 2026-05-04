# ReliefTrack

Transparent disaster fund distribution using Stellar.

## Problem
NGOs in Bulacan cannot prove how donations are distributed, reducing donor trust.

## Solution
A Soroban contract records every disbursement on-chain with recipient and amount, enabling public verification.

## Timeline
8–10 weeks MVP

## Stellar Features Used
- USDC transfers
- Soroban smart contracts
- Trustlines

## Vision and Purpose
Build trust in disaster relief funding by making every peso traceable in real time.

## Prerequisites
- Rust
- Soroban CLI (v20+)

## Build
soroban contract build

## Test
cargo test

## Deploy to Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/relieftrack.wasm \
  --source <YOUR_KEY>

## Example Invocation
Initialize:
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn init \
  --arg <ADMIN_ADDRESS>

Disburse:
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn disburse \
  --arg <ADMIN_ADDRESS> \
  --arg <RECIPIENT_ADDRESS> \
  --arg 100

Query:
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn get \
  --arg 0

## License
MIT