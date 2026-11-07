# coqui-web-channel (deprecated)

[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](https://github.com/Boriken-Dev/coqui-web-channel/blob/main/LICENSE)

**This crate is deprecated.** Its code moved to [`coqui-transport-web`](https://github.com/Boriken-Dev/coqui-transport-web), and the
channel, datagram and failure contracts it defined moved to [`coqui-transport`](https://github.com/Boriken-Dev/coqui-transport),
so one trait is shared by every transport (WebSocket, WebTransport, pipes, sockets). This crate only re-exports both, under the
names it had, so a dependent compiles unchanged for one transition. Nothing new is added here.

## What stays the same

Every public item the crate had is at its old path: `coqui_web_channel::{Channel, Datagrams, Delivery, Dialer, Kind, Failure, Phase,
Recovery, Endpoints, connect, Limits, Cancel, ...}` and the modules `frame`, `websocket`, `webtransport` and `webrtc`, behind the
same features. Its own test suite runs against this re-export.

## What differs

`Failure::recovery` is a method of the `FailureExt` trait, since an inherent method cannot be added to a type that another
crate defines: `use coqui_web_channel::FailureExt;`. The Dart package `coqui_web_channel` is now `coqui_transport_web`, in
`coqui-transport-web`.

## Moving off it

```toml
coqui-transport-web = { git = "https://github.com/Boriken-Dev/coqui-transport-web", features = ["websocket"] }
```

Then import from `coqui_transport_web` (and `coqui_transport` for the contracts). The family has no releases yet, so its crates
depend on each other by git revision. `coqui-web-channel` is deleted only when the owner decides.

## Development

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## License

MPL-2.0; see [LICENSE](https://github.com/Boriken-Dev/coqui-web-channel/blob/main/LICENSE).
