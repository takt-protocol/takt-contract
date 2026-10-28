# Contributing to Takt Protocol Contracts

Contributions to `takt-contract` are welcome. Please read and follow these guidelines to keep code quality consistent across the workspace.

## Prerequisites

Ensure your local development environment has the following tools installed:

- **Rust:** `1.84+` (stable toolchain)
- **Rust Components:** `rustfmt`, `clippy`
- **WASM Targets:** `wasm32v1-none` or `wasm32-unknown-unknown`
- **Stellar CLI:** latest release of `stellar-cli` (v22+)

Add the WASM target via `rustup`:
```bash
rustup target add wasm32v1-none
rustup target add wasm32-unknown-unknown
```

## Local Development Commands

The workspace includes a `Makefile` with shortcuts for standard build and verification tasks:

- **Run all unit & integration tests:**
  ```bash
  make test
  # or: cargo test --workspace
  ```

- **Run lints (zero warnings allowed):**
  ```bash
  make clippy
  # or: cargo clippy --workspace --all-targets -- -D warnings
  ```

- **Check code formatting:**
  ```bash
  make fmt
  # or: cargo fmt --all -- --check
  ```

- **Build release WASM binaries:**
  ```bash
  make build
  # or: stellar contract build
  ```

## Strict Coding Standards

All contributions to smart contract code must adhere strictly to these principles:

1. **No `.unwrap()` or `.expect()`:** Never use unwraps or expectations in non-test code (`src/test.rs` is exempt). Use pattern matching, `ok_or()`, or return typed `Result<T, Error>` errors.
2. **Checked Arithmetic:** Never perform raw unchecked integer operations. Use `checked_add`, `checked_sub`, `checked_mul`, or `checked_div` to prevent overflows and underflows.
3. **Storage TTL Management:** Call explicit TTL extensions (`extend_instance_ttl`, `extend_persistent_ttl`) on every state write or mutation to prevent Soroban archival.
4. **Authentication:** Every state-modifying public entrypoint must enforce authentication (`require_auth()`) on the relevant caller address.
5. **No Placeholders:** PRs containing `todo!()`, `unimplemented!()`, or mock stub logic outside dedicated test fixtures will be rejected.

## Pull Request Workflow

1. **Fork and Branch:** Create feature branches off `main` with descriptive branch prefixes:
   - `feat/feature-name`
   - `fix/bug-fix`
   - `test/test-suite`
   - `docs/documentation-update`

2. **Commit Messages:** Follow the Conventional Commits specification:
   ```text
   <type>(<scope>): <short description>
   ```
   Examples:
   - `feat(job-engine): support configurable max runs`
   - `fix(bounty-vault): correct fee calculation precision`
   - `test(keeper-registry): verify cooldown expiration check`

3. **Continuous Integration:** Ensure `cargo fmt`, `cargo clippy`, and `cargo test --workspace` pass locally. Every PR must pass the `check-and-test` GitHub Actions CI job prior to code review and merging.
