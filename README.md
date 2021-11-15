# Beacon Chain Light Client Prototype

## Overview

- Beacon chain light client implementation following a server/client model and loosely following the design document [here](https://notes.ethereum.org/@ralexstokes/HJxDMi8vY)
- The (initial) intent of this project is to only implement the client-side to be used in tandem with a server (serving [beacon APIs](https://ethereum.github.io/beacon-APIs))
- Client will request data from a server for every slot in order to stay at the tip of the chain & then perform the proper validation/processing

## Structure & Design

This project uses types defined in Lighthouse's [`consensus/types`](https://github.com/sigp/lighthouse/tree/stable/consensus/types/src) implementation and (due to dependendency/build issues, locally) re-implements (instead of importing) certain types within the [`common/eth2`](https://github.com/sigp/lighthouse/blob/stable/common/eth2/src/types.rs) crate, which provides API-specific types as a superset of `consensus` types. The project also uses aspects from [`ethereum-consensus-monitor`](https://github.com/ralexstokes/ethereum-consensus-monitor), such as a `Timer` (which implements Lighthouse's `SystemTimeSlotClock`) for keeping track of the current slot of the beacon chain.

The following provides the geenral structure:

```
├── config
│   ├── Default.toml
│   └── Development.toml
└── src
│   ├── api_client.rs
│   ├── lib.rs
│   ├── light_client_types.rs
│   ├── main.rs
│   ├── monitor.rs
│   ├── settings.rs
│   └── timer.rs
└── Cargo.toml
```

The general process is as follows:

- `settings.rs` provides the beacon chain configuration settings as well as the server URL (and eventually, port) -- unique settings can be provided as an environment variable `RUN_ENV`, which uses files defined in the `config` directory
- `timer.rs` uses configuration settings to set the `genesis_time`, `seconds_per_slot`, & `slots_per_epoch` -- this allows for client-side calls to a server for every slot
- `monitor.rs` uses the `timer.rs` to keep track of the current slot and then uses `api_client.rs` to call [beacon APIs](https://ethereum.github.io/beacon-APIs) -- the introductory phase simply calls an external Ethereum node (e.g., using Infura), but the intent is to use a local server to serve the requests
- `api_client.rs` makes calls to the server and maps the results to Ethereum types, either already defined/created by Lighthouse or "new" objects
- `light_client_types.rs` imports Lighthouse [`consensus/types`](https://github.com/sigp/lighthouse/tree/stable/consensus/types/src), includes a subset of re-implemented types from [`common/eth2`](https://github.com/sigp/lighthouse/blob/stable/common/eth2/src/types.rs), and adds new types needed for a light client (e.g., [`LightClientUpdate`](https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/sync-protocol.md#lightclientupdate))

## Commands

As the project is built out, additional functionality will be added. For now, the only available command is simply `cargo run` -- this runs the monitor and calls an API for every slot; the existing functionality is minimal and get gets block headers. To run with debug info, use `RUST_LOG=trace cargo run`, and for different configurations under `config`, use `RUN_ENV` -- e.g., `RUN_ENV=Development cargo run`

Be sure to update the example `Default.example.toml` configuration file as `Default.toml` with accurate information before running anything.
