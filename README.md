# Past-Life

CLI tool to inspect Solana accounts, transactions, and daily balances

## Features

- Fetch the current lamport and SOL balance for any address
- Trace the latest and earliest signatures for the account
- Inspect transaction details for a given signature (with parsed transfers)
- List all transfers that touched the account on a specific day
- Reconstruct end-of-day balances and track the highest balance seen

## Requirements

- Rust 1.70+ (stable toolchain)
- Internet access to a Solana RPC endpoint (default: `https://api.mainnet-beta.solana.com`)

## Build & Run

```bash
cargo run
```

The CLI will prompt for the account address and menu options such as:

```
Enter the account you want to query:
...
1. Get info on a transaction at specific signature
2. Get account balance at specific date
3. Get historical account balance
4. Get account transactions at specific date
5. Exit
```

## Example Output

```
######################GET ACCOUNT BALANCE#########################
Enter the account you want to query:
68LLZMjsoSxgY5C7P8vd7mJLuiD8idAWpAXRinhkdApi
This account has: 2710060 lamport
Which is 0.00271006 in solana
...
2025-09-16 | 4MFFfH... | 2710060 lamports (0.002710060 SOL)
2025-09-15 | 4F2Vqh... | 1780040 lamports (0.001780040 SOL)
```

## Roadmap / Ideas

- CSV export of daily balances
- Retry/backoff for RPC requests
- Support for custom RPC endpoints via CLI args

## License

MIT License. See [LICENSE](LICENSE).
