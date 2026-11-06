# Security Policy

## Audit Status Disclaimer

`takt-contract` (v0.1.0) is currently unaudited and deployed on Stellar Testnet for testing and evaluation purposes. Do not deposit production mainnet funds until a formal third-party security audit has been completed and published.

## Scope

The following contracts in this repository are in scope for security vulnerability reports:

- `contracts/keeper-registry`
- `contracts/bounty-vault`
- `contracts/job-engine`

Out-of-scope items include third-party dependencies (such as the Stellar core protocol or Soroban SDK itself) and external systems not maintained in this repository.

## Threat Model & Architectural Surface

### 1. Arbitrary Cross-Contract Invocation (`job-engine::execute_job`)
- `create_job` allows any user to specify a `target_contract`, `target_fn`, and `target_args`. When `execute_job` runs, `job-engine` invokes `env.invoke_contract::<Val>(&job.target_contract, &job.target_fn, job.target_args)`.
- **Security Boundary:** Because `job-engine` is the authorized caller inside `keeper-registry` (`record_execution`) and `bounty-vault` (`payout_execution`, `fund_job_from_engine`), a malicious job owner could attempt to set `target_contract` to `keeper-registry` or `bounty-vault` to trigger privileged state changes.
- **Mitigation & Open Hardening Issue:** Downstream target contracts must never rely solely on `job-engine` caller identity for privileged user actions. Explicit target-address denylisting (`target_contract != keeper_registry && target_contract != bounty_vault && target_contract != env.current_contract_address()`) is tracked as a priority security hardening issue.

### 2. Admin Key Centralization & Slashing (`keeper-registry::slash_keeper`)
- Administrative functions including `slash_keeper` and `set_engine` currently require `config.admin.require_auth()`. Compromising the admin key could slash active keepers or redirect engine authorization.
- Admin controls are currently held by a single Testnet key (`GBKGRYZZEELD6POCG5E3MNMA3CT7OKF45CUX3FUDSJH6NJXAMTPXIYRS`), with transition to multi-sig / timelock governance planned on the roadmap.

### 3. Target Revert & DoS on Keeper Execution
- If a target contract panics or traps (e.g., mismatched argument arity or out-of-gas), the entire `execute_job` transaction reverts atomically on Soroban.
- Keepers incur simulation and inclusion overhead if they broadcast transactions without pre-simulating execution via Soroban RPC (`simulateTransaction`). Keepers should simulate target executions prior to on-chain submission to prevent fee exhaustion.

## Responsible Disclosure

If you discover a security vulnerability or exploit vector within `takt-contract`, please disclose it responsibly. Do NOT open a public issue.

Submit vulnerability reports privately through GitHub Security Advisories:
[Submit a Security Advisory](https://github.com/takt-protocol/takt-contract/security/advisories/new)
