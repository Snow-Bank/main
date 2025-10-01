# Snowbank (Solana / Anchor)

Port of the Snowbank EVM protocol to **Solana** using **Anchor**. This repository contains:
- An Anchor program (`programs/snowbank`) implementing config, presale, vesting, and farm logic.
- A minimal React/TypeScript frontend (`app/`) that connects to Phantom, loads the IDL, and calls sample instructions.
- Developer docs: `DEVGUIDE.md`, `CHANGEME.md`, `CHANGELOG.md`.

> Generated: 2025-10-01

## Quick Start (Localnet)

```bash
# prerequisites
solana --version        # 1.18+
anchor --version        # 0.30+
node --version          # 18+
yarn --version          # 1.22+

# 1) start a local validator
solana-test-validator -r

# 2) build & deploy the program
anchor build
anchor deploy

# 3) launch the frontend
cd app
yarn
yarn dev
```

## Structure
```
snowbank-solana/
  Anchor.toml
  programs/snowbank/...
  tests/snowbank.spec.ts
  app/...
  README.md
  DEVGUIDE.md
  CHANGEME.md
  CHANGELOG.md
```

See **DEVGUIDE.md** for full details and **CHANGEME.md** for the minimal fields to edit before mainnet.
