//! key-mapping library allows for keyboard key code conversion between systems such as the DOM and
//! HID usage-ids. With Rust `[no_std]` support.
//!
//! # Features
//!
//! Extra functionality is behind optional features to optimize compile time and binary size.
//!
//! - **`std`** *(enabled by default)* - Add support for Rust's libstd types.
//! - **`serde`** Add support for `serde` de/serializing library..
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Keyboard layouts, used to convert between key-code types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Keyboard {
    #[default]
    US,
    UK,
}

impl Keyboard {
    /// Convert key-code into a hid usage id, using the given keyboard layout.
    /// Uses a performant O(1) operation.
    pub fn dom_key_to_usage_id(&self, key_code: &str) -> Option<&u8> {
        match self {
            Self::US => DOM_KEYS_US.get(key_code),
            Self::UK => DOM_KEYS_UK.get(key_code),
        }
    }
}

/// A single mapped keyboard key.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MappedKey<'a> {
    /// HID usage-id for keyboard key
    pub usage_id: u8,
    /// The DOM key representation
    pub dom_key: &'a str,
    /// Machine friendly key name
    pub prefix: &'a str,
}

#[cfg(test)]
mod tests {
    use crate::Keyboard;

    #[test]
    fn dom_key_to_hid() {
        assert_eq!(0x04, *Keyboard::US.dom_key_to_usage_id("KeyA").unwrap());
        assert_eq!(
            0x31,
            *Keyboard::US.dom_key_to_usage_id("Backslash").unwrap()
        );
        assert_eq!(
            0x32,
            *Keyboard::UK.dom_key_to_usage_id("Backslash").unwrap()
        );
    }
}
