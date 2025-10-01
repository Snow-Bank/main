# CHANGEME — One-pass checklist before going live

- [ ] Update **program id** in `programs/snowbank/src/lib.rs` and `Anchor.toml`.
- [ ] Set **treasury** and **owner** pubkeys in `ix/admin.rs::InitArgs` before `init_global`.
- [ ] Decide on **SNOW mint decimals** (6 or 9). Update in `ix/admin.rs` when creating the mint.
- [ ] Set `timelock_seconds` (43200 mainnet) in `init_global`.
- [ ] Configure **emission_per_sec** per pool via `set_emission_per_sec_tokens`.
- [ ] Tune **deposit_fee_bps** and enable/disable **sweeps** as desired.
- [ ] Review presale **base_tokens_per_sol**, **step_lamports**, and **step_percent_bps**.
- [ ] Re-audit **PDAs & signer seeds** in all ix contexts.
- [ ] Run `anchor test` with your parameters.
- [ ] Rebuild the frontend `.env` and `snowbank.config.json` with deployed addresses.
