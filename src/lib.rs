//! Library to assist with common SAE-J1939 conversions.
//!
//! # Features
//!
//! - Standard and extended identifier types.
//! - Conversion to and from raw bit reperesntations.
//! - STM32 `bxcan` interop support.

#![no_std]
#![allow(dead_code)]

mod pdu;
pub use crate::pdu::*;

mod value;
pub use crate::value::*;

mod identifier;
pub use crate::identifier::*;
