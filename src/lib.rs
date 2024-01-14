//! # rust-ocpp
//!
//! `rust-ocpp` is a collection of data types and enums specified by the Open Charge Point Protocol
//! by Open Charge Alliance
//!
//! More Info on Open Charge Alliance can be found here: [Open Charge Alliance](https://www.openchargealliance.org/)
//!
//! It aims to implement the `ocpp 1.6` and `ocpp 2.0.1` protocols.
//!
//! # v1_6
//! The [v1_6](v1_6) module contains the `ocpp 1.6` implementation
//!
//! # v2_0_1
//! The [v2_0_1](v2_0_1) module contains the `ocpp 2.0.1` implementation

#![cfg_attr(not(feature = "std"), no_std)]

/// tests
#[cfg(all(test, feature = "std"))]
pub mod tests;
#[cfg(feature = "v1_6")]
pub mod v1_6;
#[cfg(feature = "v2_0_1")]
pub mod v2_0_1;

mod date_time;

#[cfg(feature="std")]
mod std_vec;
#[cfg(feature="std")]
pub use std_vec::Vec;

#[cfg(feature="no_std")]
mod heapless_vec;
#[cfg(feature="no_std")]
pub use heapless_vec::Vec;
