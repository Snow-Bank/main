# Developer Guide

This guide explains how the EVM components map onto Solana/Anchor and how to extend the skeleton.

## Mapping (EVM → Solana)

- Contracts → Programs (Anchor). We provide a single program `snowbank` with modules.
- ERC20 → SPL Token. SNOW is an SPL mint with PDA mint authority.
- MasterChef → Custodial vault PDAs + on-harvest minting.
- Timelock → PDA queue with execute timestamps.
- Presale → SOL-based sale that credits vesting grants.
- Vesting → Per-user grants with launch-claim and interval-claim rules.
- NFT/Marketplace → Metaplex-compatible mint + optional in-house escrow (not included in initial skeleton).

## Code Layout

- `programs/snowbank/src/state/*`: PDA account definitions
- `programs/snowbank/src/ix/*`: instruction handlers
- `programs/snowbank/src/util/*`: math and token helpers
- `programs/snowbank/src/errors.rs`: program error codes
- `programs/snowbank/src/lib.rs`: Anchor entrypoints

## Building and Testing

```bash
anchor build
anchor test
```

- Tests live in `tests/snowbank.spec.ts`. Add more to cover presale tranches, referral liabilities, vesting windows, and farming.

## Extending

- Add more pools: call `add_pool` per stake mint.
- Add USDC presale: extend `presale::buy` to accept a token payment via CPI and pricing in stable units.
- Add Jupiter-based zaps: do it client-side in the frontend using the Jupiter SDK and compose transactions to farm deposit.
