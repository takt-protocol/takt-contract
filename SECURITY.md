# Security Policy

## Audit Status Disclaimer

`takt-contract` (v0.1.0) is currently unaudited and deployed on Stellar Testnet for testing and evaluation purposes. Do not deposit production mainnet funds until a formal third-party security audit has been completed and published.

## Scope

The following contracts in this repository are in scope for security vulnerability reports:

- `contracts/keeper-registry`
- `contracts/bounty-vault`
- `contracts/job-engine`

Out-of-scope items include third-party dependencies (such as the Stellar core protocol or Soroban SDK itself) and external systems not maintained in this repository.

## Responsible Disclosure

If you discover a security vulnerability or exploit vector within `takt-contract`, please disclose it responsibly. Do NOT open a public issue.

Submit vulnerability reports privately through GitHub Security Advisories:
[Submit a Security Advisory](https://github.com/takt-protocol/takt-contract/security/advisories/new)

### Response SLA

- **Acknowledgment:** Within 48 hours of initial report submission.
- **Assessment & Triage:** An initial severity assessment and triage update will be shared within 5 business days.
- **Resolution & Disclosure:** Patches will be developed, tested, and deployed to test environments before coordinated public disclosure.
