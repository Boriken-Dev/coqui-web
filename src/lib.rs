#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// `coqui-transport`, where the channel, datagram and failure contracts and the framing codecs are defined.
pub use coqui_transport;
/// `coqui-transport-web`, where this crate's code moved.
pub use coqui_transport_web;
/// Everything this crate exported before it moved, under the names it had, so a dependent compiles unchanged.
pub use coqui_transport_web::*;
