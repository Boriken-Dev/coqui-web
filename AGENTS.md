# `coqui-web-channel` — API header (deprecated)

A re-export crate: it has no code of its own. Its code moved to `coqui-transport-web` (<https://github.com/Boriken-Dev/coqui-transport-web>),
and the channel contract to `coqui-transport` (<https://github.com/Boriken-Dev/coqui-transport>). Read their headers for the API;
this one says only what this crate adds and what differs. It ships inside the package and is self-contained.

| Feature | Forwards to | Default |
| --- | --- | --- |
| `websocket` | `coqui-transport-web/websocket` | off |
| `webtransport` | `coqui-transport-web/webtransport` | off |
| `webrtc` | `coqui-transport-web/webrtc`; implies both others | off |

MSRV Rust 1.89. Version `0.0.0-alpha.0`, not published. No `unsafe` code.

## Public items

```rust
pub use coqui_transport;                  // the contracts and codecs
pub use coqui_transport_web;              // the code that moved
pub use coqui_transport_web::*;           // every name this crate had, at its old path
```

The old paths are `Channel`, `Datagrams`, `Delivery`, `Dialer`, `Kind`, `Preflight`, `NoPreflight`, `Unverified`, `connect`,
`CertificatePin`, `Endpoints`, `EndpointsBuilder`, `EndpointError`, `Failure`, `Phase`, `Recovery`, `Limits` and `Cancel` at the root, and
the modules `frame`, `websocket`, `webtransport` and `webrtc`. Plus what `coqui-transport-web` has added since: `Capabilities`,
`Carrier`, `FailureExt`, `NonDefault`.

## Differences from the crate before it moved

- `Failure::recovery(failed, phase, has_websocket)` is a method of the `FailureExt` trait: `use coqui_web_channel::FailureExt;`.
- `Channel`, `Datagrams`, `Delivery` and `Failure` are `coqui_transport`'s types. Two dependents that used this crate and
  `coqui-transport` now share one trait; one that used a copy before had two.
- The Dart package moved with the code, as `coqui_transport_web`.

## Gotchas

- Keep its tests unchanged: they are the proof that the re-export is complete. A test that needs a new import means an
  incompatible change, which belongs in `coqui-transport-web`'s changelog.
- The dependency on `coqui-transport-web` is a git revision. Move it with `coqui-transport` so one copy of the core crate is in a
  dependent's graph.
- Marked deprecated in its description, README and docs only: a `#[deprecated]` attribute would make every dependent's
  `-D warnings` build fail on the day it lands, which is not a transition.
