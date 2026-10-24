# RestoreLab continuity specimen

A small Soroban contract designed to make storage lifetime differences observable. The constructor sets an owner in instance storage. Owner-authorized `put` writes a 32-byte digest to persistent storage, while `hint` writes a temporary digest. Public `record` and `read_hint` return optional values. Permissionless `maintain(extend_to)` extends instance/code and the durable record, bounded by the host's maximum TTL; it deliberately leaves temporary hints untouched. Anyone paying rent may extend lifetime; only the owner may change the stored digests.

Seven Rust tests exercise authorization, durable round-trip, temporary expiration, selective TTL maintenance, invalid extension and persistent access after expiry in the host environment. Host expiration tests are separate from the live testnet pilot, which demonstrates extension of live entries, not restoration of already archived entries.

```sh
cargo test --locked
stellar contract build --locked
```

Rust 1.97.1 and Stellar CLI 28.1.0 were used locally. SDK 29.0.1 is pinned. Use the supported wasm32v1-none target through Stellar CLI so contract specification processing runs. The deployed testnet executable and digest are recorded in the backend's testnet-evidence.json. This specimen is a teaching/test integration surface, not a backup service or production data store.

## Reference

[Stellar state archival](https://developers.stellar.org/docs/learn/fundamentals/contract-development/storage/state-archival) · [RPC live entry lookup](https://developers.stellar.org/docs/data/apis/rpc/api-reference/methods/getLedgerEntries)
