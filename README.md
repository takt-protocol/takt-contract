<p align="center">
  <img src="assets/logo.svg" alt="Takt Protocol" width="800" />
</p>

# Takt Protocol — `takt-contract`

[![CI](https://github.com/takt-protocol/takt-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/takt-protocol/takt-contract/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Decentralized task scheduling and bounty-incentivized keeper execution primitive on Stellar Soroban.

---

## Overview

Soroban smart contracts cannot self-execute on a scheduled basis, nor can they react autonomously to time without external invocation. Additionally, idle state data in Soroban risks being archived if storage time-to-live (TTL) thresholds lapse.

**Takt Protocol** provides on-chain infrastructure for trust-minimized, automated contract execution on Stellar:
- **Decentralized Task Scheduling:** Register repetitive or one-off tasks with custom intervals and execution constraints.
- **Dynamic Cross-Contract Invocations:** Invocations pass arbitrary contract targets, function symbols, and argument vectors safely via `env.invoke_contract`.
- **SEP-41 Escrow & Bounty Settlement:** Users fund per-job escrows in any SEP-41 token, rewarding decentralized keepers while deducting a protocol fee into the treasury.
- **Keeper Economic Security:** Keepers stake tokens with mandatory unbonding cooldown periods and are subject to admin slashing for malicious behavior.

---

## Repository Architecture & Dependency Order

The protocol is partitioned into 3 isolated Soroban contracts within a single Cargo workspace, deployed in strict dependency order:

```
keeper-registry (1) <---+
                        |
bounty-vault    (2) <---+--- job-engine (3)
```

| Order | Contract | Path | Role & Responsibilities |
|---|---|---|---|
| **1** | `keeper-registry` | [`contracts/keeper-registry`](contracts/keeper-registry) | Manages keeper stakes, unbonding cooldown timers, keeper status tracking, execution counters, and admin slashing. |
| **2** | `bounty-vault` | [`contracts/bounty-vault`](contracts/bounty-vault) | Custodies SEP-41 deposit balances, locks per-job bounty escrows, calculates basis-point fee splits, executes payouts to keepers, and refunds unspent escrows. |
| **3** | `job-engine` | [`contracts/job-engine`](contracts/job-engine) | Orchestrates job registration, interval and cadence enforcement, dynamic target contract invocations, lifecycle transitions (active, paused, completed, cancelled), and execution receipt storage. |

---

## Deployed Stellar Testnet Contracts

- **Network:** Stellar Testnet (`Test SDF Network ; September 2015`)
- **Admin / Treasury:** `GBKGRYZZEELD6POCG5E3MNMA3CT7OKF45CUX3FUDSJH6NJXAMTPXIYRS`

| Contract | Address | Explorer |
|---|---|---|
| `keeper-registry` | `CDSBDXYELFFHM7XMG4D4BXEVETOHQDWPQOTQLPRHGTTVW7DTNODHIIZN` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CDSBDXYELFFHM7XMG4D4BXEVETOHQDWPQOTQLPRHGTTVW7DTNODHIIZN) |
| `bounty-vault` | `CDDIJAZCXR7TWSTI4B2FSE7MCXFKH2JVOBT4WELSSJLZHV5XQ4PKHEGO` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CDDIJAZCXR7TWSTI4B2FSE7MCXFKH2JVOBT4WELSSJLZHV5XQ4PKHEGO) |
| `job-engine` | `CC6ZSDYSGFAZDJ2FP2RT2ZV6NUZN4OFEWYNIS3CYOG2CBG7DIJDCWYOT` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CC6ZSDYSGFAZDJ2FP2RT2ZV6NUZN4OFEWYNIS3CYOG2CBG7DIJDCWYOT) |
| `token` (Wrapped Native XLM) | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` | [Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC) |

### Testnet Deployment Log

From the maintainer's testnet deployment log:

| Operation | Transaction Hash | Explorer |
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

---

## Project Status & Current Limitations

- **Stage:** Alpha / Stellar Testnet Only (`Test SDF Network ; September 2015`).
- **Audit Status:** Unaudited. Do not deploy with mainnet funds until formal security review is completed.
- **Trust Assumptions & Roadmap:**
  - `initialize`, `set_engine`, and `slash_keeper` are currently governed by a single configured `admin` address (`GBKGRYZZEELD6POCG5E3MNMA3CT7OKF45CUX3FUDSJH6NJXAMTPXIYRS` on Testnet). Migrating admin controls to a multi-sig / timelock contract is planned on the roadmap.
  - Target contract execution (`execute_job`) dispatches cross-contract calls (`env.invoke_contract`) without caller reentrancy guards beyond Soroban's native host frame isolation.

---

## Contract Entrypoints Reference

### `KeeperRegistry`

| Function | Access | Description |
|---|---|---|
| `initialize(admin, stake_token, min_stake, unbond_period)` | Admin | Sets initial governance parameters and staking token address. |
| `set_engine(engine)` | Admin | Authorizes the `JobEngine` contract to call internal registry methods. |
| `register_keeper(keeper, amount)` | Keeper | Stakes tokens to register as an active keeper. |
| `top_up_stake(keeper, amount)` | Keeper | Adds additional stake to an existing active registration. |
| `request_unbond(keeper)` | Keeper | Initiates the unbonding cooldown period. |
| `withdraw_stake(keeper)` | Keeper | Withdraws staked tokens once the cooldown period has elapsed. |
| `record_execution(keeper)` | Engine Only | Increments keeper execution count and updates last active timestamp. |
| `slash_keeper(keeper, penalty, recipient)` | Admin | Slashes a keeper's stake for misconduct and transfers penalty tokens. |
| `get_keeper(keeper)` | Public Read | Returns keeper profile struct (stake, status, execution count, cooldowns). |
| `is_active_keeper(keeper)` | Public Read | Checks whether an address is currently an authorized active keeper. |
| `get_total_keepers()` | Public Read | Returns the total count of registered keepers. |

### `BountyVault`

| Function | Access | Description |
|---|---|---|
| `initialize(admin, token, fee_bps, treasury)` | Admin | Initializes vault with supported token, fee basis points, and treasury address. |
| `set_engine(engine)` | Admin | Authorizes `JobEngine` to lock and payout job escrows. |
| `deposit(from, amount)` | User | Deposits SEP-41 tokens into the user's vault balance. |
| `withdraw(to, amount)` | User | Withdraws available (unlocked) tokens back to the user's wallet. |
| `fund_job_from_engine(owner, job_id, amount)` | Engine Only | Moves user's deposit balance into an escrow dedicated to a specific job. |
| `payout_execution(job_id, keeper, bounty)` | Engine Only | Deducts bounty from job escrow, sends protocol fee to treasury, and pays keeper reward. |
| `refund_job(job_id, recipient)` | Engine Only | Returns all unspent job escrow back to the owner's vault balance upon cancellation. |
| `get_user_balance(user)` | Public Read | Returns free, unallocated balance of a user. |
| `get_job_escrow(job_id)` | Public Read | Returns current escrow balance locked for a job. |

### `JobEngine`

| Function | Access | Description |
|---|---|---|
| `initialize(admin, keeper_registry, bounty_vault)` | Admin | Configures linked registry and vault addresses and initializes ID counters. |
| `create_job(...)` | Job Owner | Locks initial escrow and registers a job with recurrence intervals and target call args. |
| `execute_job(keeper, job_id)` | Active Keeper | Verifies keeper and intervals, triggers target execution, updates receipt, and triggers payout. |
| `pause_job(owner, job_id)` | Job Owner | Pauses an active job to halt executions temporarily. |
| `resume_job(owner, job_id)` | Job Owner | Resumes execution eligibility for a paused job. |
| `cancel_job(owner, job_id)` | Job Owner | Permanently cancels a job and triggers full refund of unspent escrow. |
| `is_executable(job_id)` | Public Read | Evaluates whether a job is eligible for immediate execution (timing, status, escrow). |
| `get_job(job_id)` | Public Read | Retrieves full job configuration and current lifecycle state. |
| `get_last_receipt(job_id)` | Public Read | Retrieves the most recent execution receipt for a job. |
| `get_total_jobs()` | Public Read | Returns total number of jobs created to date. |

---

## Quick Start

### Build Contracts

Compile all contracts into Soroban WASM targets:
```bash
stellar contract build
```

### Run Tests

Run all unit tests and end-to-end integration tests across the workspace:
```bash
cargo test --workspace
```

### Run Linter & Formatter

Verify code formatting and clippy rules without warnings:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

### Deploy to Stellar Testnet

Run the automated testnet deployment script to deploy and link all contracts in sequence:
```bash
./scripts/deploy_testnet.sh
```

---

## Related Repositories

- [`takt-protocol/takt-app`](https://github.com/takt-protocol/takt-app): Web dashboard, TypeScript SDK, event indexer, and autonomous keeper node client for the Takt Protocol.

---

## Maintainers

| Maintainer | Role | Contact |
|---|---|---|
| webdevayo | Core Maintainer | [@dev_ayo](https://t.me/dev_ayo) |

---

## Community

- [Community Chat](https://t.me/+9120mKpa2CpkNjg8)

---

## Contributors

Thank you to everyone contributing to Takt Protocol!

<a href="https://github.com/takt-protocol/takt-contract/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=takt-protocol/takt-contract" alt="Contributors" />
</a>

---

## License

This project is licensed under the [MIT License](LICENSE).
