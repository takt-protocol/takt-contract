# Takt Protocol Contracts v0.1.0 (Testnet Alpha)

## Summary

This is the initial Testnet Alpha release of `takt-contract` (`takt-protocol/takt-contract`). It provides the core smart contract primitives for recurring task scheduling and bounty-settled keeper execution on Stellar Soroban.

The workspace contains three contracts:
- `keeper-registry`: Manages keeper stakes, unbonding cooldowns, execution telemetry, and administrative slashing.
- `bounty-vault`: Custodies SEP-41 deposit collateral, locks per-job bounty escrows, calculates basis-point fee splits, executes payouts to keepers, and refunds unspent escrows.
- `job-engine`: Orchestrates job registration, interval enforcement, dynamic target invocations (`env.invoke_contract`), lifecycle state management, and execution receipts.

## Disclaimer

> [!WARNING]
> This release is unaudited and intended strictly for testing and evaluation on Stellar Testnet (`Test SDF Network ; September 2015`). Do not deploy these contracts to Stellar Mainnet or deposit production funds.

## Commit Reference

- **Release Commit SHA:** `3ee2c89b0d35ce8cfac16a557c0b1663ed1489a4`
- **Network:** Stellar Testnet (`Test SDF Network ; September 2015`)
- **Admin / Treasury Address:** `GBKGRYZZEELD6POCG5E3MNMA3CT7OKF45CUX3FUDSJH6NJXAMTPXIYRS`

## Deployed Testnet Contracts

| Contract | Address | Explorer Link |
|---|---|---|
| `keeper-registry` | `CDSBDXYELFFHM7XMG4D4BXEVETOHQDWPQOTQLPRHGTTVW7DTNODHIIZN` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CDSBDXYELFFHM7XMG4D4BXEVETOHQDWPQOTQLPRHGTTVW7DTNODHIIZN) |
| `bounty-vault` | `CDDIJAZCXR7TWSTI4B2FSE7MCXFKH2JVOBT4WELSSJLZHV5XQ4PKHEGO` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CDDIJAZCXR7TWSTI4B2FSE7MCXFKH2JVOBT4WELSSJLZHV5XQ4PKHEGO) |
| `job-engine` | `CC6ZSDYSGFAZDJ2FP2RT2ZV6NUZN4OFEWYNIS3CYOG2CBG7DIJDCWYOT` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CC6ZSDYSGFAZDJ2FP2RT2ZV6NUZN4OFEWYNIS3CYOG2CBG7DIJDCWYOT) |
| `token` (Wrapped Native XLM) | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC) |

## Testnet Deployment Transactions

From the maintainer's testnet deployment log:

| Operation | Transaction Hash | Explorer Link |
|---|---|---|
| keeper-registry deploy and init | `00f576872aee9a7ac6332174cdec8c659ac4400d4ce36f1b00c843e67958f8af` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/00f576872aee9a7ac6332174cdec8c659ac4400d4ce36f1b00c843e67958f8af) |
| bounty-vault deploy and init | `b9a9b1be24970b636905ac9015428c1d9763017798018a226e27ba398c6fd8fc` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/b9a9b1be24970b636905ac9015428c1d9763017798018a226e27ba398c6fd8fc) |
| job-engine deploy and init | `d2401b75adda5f36e1d8a1396cacffbba5d1bd013e085e36d423f10b7d5adaab` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/d2401b75adda5f36e1d8a1396cacffbba5d1bd013e085e36d423f10b7d5adaab) |
| set_engine in registry | `981ca837efcaaa93f462075fc5801d3d4b307b1bbb30502d920799e02e73cd48` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/981ca837efcaaa93f462075fc5801d3d4b307b1bbb30502d920799e02e73cd48) |
| set_engine in vault | `d698300e25a812389cd3c7c2b6057cdbe8c9f1f5238101b446318f6cb2d6122d` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/d698300e25a812389cd3c7c2b6057cdbe8c9f1f5238101b446318f6cb2d6122d) |
| register keeper (stake 100 XLM) | `b2d2a818bf3a099e38e366f2891c495ff701cb07bebed10fb81b8a26f0756adf` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/b2d2a818bf3a099e38e366f2891c495ff701cb07bebed10fb81b8a26f0756adf) |
| vault deposit | `e04408446bb6063f4328dcdb2c48f79d6b12061d9f84733a88d1f5d01c295e7a` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/e04408446bb6063f4328dcdb2c48f79d6b12061d9f84733a88d1f5d01c295e7a) |
| create_job #1 (revert test case) | `94aac3b86a2637538de46574e4050492c244e7d4921359897239cfe29fb8458a` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/94aac3b86a2637538de46574e4050492c244e7d4921359897239cfe29fb8458a) |
| create_job #2 | `a266c3c9ce3a9603e413497ee6fffa9fb661288516e8ff26ba6e92951bca0bbd` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/a266c3c9ce3a9603e413497ee6fffa9fb661288516e8ff26ba6e92951bca0bbd) |
| execute_job #2 (keeper paid 9,500,000 stroops, protocol fee 500,000) | `d7e6b7284b7e1ac8ef297b994bb387aaffafae4fff518647435e07450044e4b5` | [Stellar.Expert](https://stellar.expert/explorer/testnet/tx/d7e6b7284b7e1ac8ef297b994bb387aaffafae4fff518647435e07450044e4b5) |

From the maintainer's testnet deployment log, Job #2 is a live example (target keeper-registry::get_total_keepers, 60 s interval, 1 XLM bounty, 20 XLM escrow).

## Known Limitations

- **Stage:** Alpha / Stellar Testnet Only (`Test SDF Network ; September 2015`).
- **Audit Status:** Unaudited. Do not deploy with mainnet funds until formal security review is completed.
- **Trust Assumptions & Roadmap:**
  - `initialize`, `set_engine`, and `slash_keeper` are currently governed by a single configured `admin` address (`GBKGRYZZEELD6POCG5E3MNMA3CT7OKF45CUX3FUDSJH6NJXAMTPXIYRS` on Testnet). Migrating admin controls to a multi-sig / timelock contract is planned on the roadmap.
  - Target contract execution (`execute_job`) verifies keeper and interval eligibility, records execution in `keeper-registry`, dispatches the cross-contract call (`env.invoke_contract`), and triggers bounty payout via `bounty-vault`.

## Build and Verification

### Prerequisites

- Rust 1.84+
- `wasm32v1-none` or `wasm32-unknown-unknown` target
- Stellar CLI v22+

### Verification Commands

```bash
# Verify formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings

# Run all unit and integration tests
cargo test --workspace

# Compile WASM targets
stellar contract build
```
