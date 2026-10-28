# Takt Protocol — `takt-contract`

[![CI](https://github.com/takt-protocol/takt-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/takt-protocol/takt-contract/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Soroban SDK](https://img.shields.io/badge/Soroban%20SDK-22.0.0-purple.svg)](https://crates.io/crates/soroban-sdk)

Decentralized automation engine, keeper registry, and escrowed bounty vault built on the Stellar Soroban smart contract platform.

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

## Contributors

Thank you to everyone contributing to Takt Protocol!

<a href="https://github.com/takt-protocol/takt-contract/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=takt-protocol/takt-contract" alt="Contributors" />
</a>

---

## License

This project is licensed under the [MIT License](LICENSE).
