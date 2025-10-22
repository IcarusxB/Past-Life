# Past-Life

CLI tool to inspect Solana accounts, transactions, and balances.

## Features

- Fetch current lamport and SOL balances for any address
- Trace latest and earliest signatures for the account
- Inspect transaction details for a given signature
- List all transfers involving the account on a specific day
- Reconstruct end-of-day balances and highlight the highest balance seen

## Build & Run

```bash
cargo run
```

The CLI prompts for an address and presents a menu such as:
```bash
1. Get info on a transaction at specific signature
2. Get historical account balance
3. Get account transactions at specific date
4. Exit
```
