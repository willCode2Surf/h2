//#![deny(warnings, missing_debug_implementations, missing_docs)]

//! HTTP2

#[macro_use]
extern crate futures;

#[macro_use]
extern crate tokio_io;

// HTTP types
extern crate http;

// Buffer utilities
extern crate bytes;

// Hash function used for HPACK encoding and tracking stream states.
extern crate fnv;

extern crate byteorder;
extern crate slab;

#[macro_use]
extern crate log;
extern crate string;
extern crate ordermap;

mod error;
#[cfg_attr(feature = "unstable", allow(missing_docs))]
mod codec;
mod hpack;
mod proto;

#[cfg(not(feature = "unstable"))]
mod frame;

#[cfg(feature = "unstable")]
#[allow(missing_docs)]
pub mod frame;

pub mod client;
pub mod server;
mod share;

pub use error::{Error, Reason};
pub use share::{SendStream, RecvStream, ReleaseCapacity};

#[cfg(feature = "unstable")]
pub use codec::{Codec, RecvError, SendError, UserError};
