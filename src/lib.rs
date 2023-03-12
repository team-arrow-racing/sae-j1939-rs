//! Library to assist with common SAE-J1939 conversions.
//!
//! # Features
//!
//! - Standard and extended identifier types.
//! - Conversion to and from raw bit reperesntations.
//! - STM32 `bxcan` interop support.

#![no_std]
#![allow(dead_code)]

mod identifier;
mod pgn;
mod value;

pub use crate::identifier::*;
pub use crate::pgn::*;
pub use crate::value::*;
