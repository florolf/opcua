//! The OPC UA Client module provides the functionality necessary for a client to connect to an OPC UA server,
//! authenticate itself, send messages, receive responses, get values, browse the address space and
//! provide callbacks for things to be propagated to the client.

#[macro_use]
extern crate log;
extern crate url;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate opcua_types;
extern crate opcua_core;

mod comms;

pub mod config;
pub mod client;
pub mod session;
pub mod subscription;

pub mod prelude {
    pub use opcua_core::prelude::*;
    pub use client::*;
    pub use config::ClientConfig;
    pub use session::*;
    pub use subscription::*;
}

#[cfg(test)]
mod tests;