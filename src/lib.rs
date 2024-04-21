#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! key-mapping library allows for keyboard key code conversion between systems such as the DOM and
//! HID usage-ids. With Rust `[no_std]` support.
//!
//! # Features
//!
//! Extra functionality is behind optional features to optimize compile time and binary size.
//!
//! - **`std`** *(enabled by default)* - Add support for Rust's libstd types.
//! - **`serde`** Add support for `serde` de/serializing library.
//! - **`usbd-hid`** Add support for converting between the usbd-hid library KeyboardReport.
//!
//! # Example Usage
//!
//! ```toml
//! [dependencies]
//! key-mapping = "0.2"
//! ```
//!
//! ```rust,editable
//! use key_mapping::Keyboard;
//!
//! fn main() {
//!     let dom_code = "KeyA";
//!     let usage_id = Keyboard::US.dom_key_to_usage_id(dom_code).unwrap();
//!
//!     assert_eq!(0x04, *usage_id);
//! }
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub const MODIFIER_CODE_CTRL: u8 = 1;
pub const MODIFIER_CODE_SHIFT: u8 = 2;
pub const MODIFIER_CODE_ALT: u8 = 4;
pub const MODIFIER_CODE_META: u8 = 8;

/// Keyboard layouts, used to convert between key-code types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Keyboard {
    /// US keyboard layout *(default)*
    #[default]
    US,
    /// UK keyboard layout
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MappedKey<'a> {
    /// HID usage-id for keyboard key
    pub usage_id: u8,
    /// The DOM key representation
    pub dom_key: &'a str,
    /// Machine friendly key name
    pub prefix: &'a str,
}

/// A keyboard action, could be used for making key press/release events,
/// Defaults to no keys or modifiers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyboardAction {
    /// Keys included in action, represented as usage-ids
    pub keys: [Keys; 6],
    /// Whether ALT is held
    pub alt: bool,
    /// Whether CTRL is held
    pub ctrl: bool,
    /// Whether SHIFT is held
    pub shift: bool,
    /// Whether META is held
    pub meta: bool,
}

impl Default for KeyboardAction {
    fn default() -> Self {
        Self {
            keys: [
                Keys::None,
                Keys::None,
                Keys::None,
                Keys::None,
                Keys::None,
                Keys::None,
            ],
            alt: Default::default(),
            ctrl: Default::default(),
            shift: Default::default(),
            meta: Default::default(),
        }
    }
}

#[cfg(feature = "usbd-hid")]
impl From<KeyboardAction> for usbd_hid::descriptor::KeyboardReport {
    fn from(value: KeyboardAction) -> Self {
        let mut keycodes = [0; 6];
        for (i, v) in value.keys.into_iter().map(|v| v as u8).enumerate() {
            keycodes[i] = v;
        }
        Self {
            modifier: value.get_modifer_code(),
            reserved: 0,
            leds: 0,
            keycodes,
        }
    }
}

impl KeyboardAction {
    /// Get the modifiers as their code representation
    pub fn get_modifer_code(&self) -> u8 {
        let mut result = 0;
        if self.ctrl {
            result |= MODIFIER_CODE_CTRL;
        }
        if self.shift {
            result |= MODIFIER_CODE_SHIFT;
        }
        if self.alt {
            result |= MODIFIER_CODE_ALT;
        }
        if self.meta {
            result |= MODIFIER_CODE_META;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{Keyboard, MappedKey, MAPPED_KEYS};

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

    #[test]
    fn usage_id_to_mapping() {
        assert_eq!(
            MAPPED_KEYS.get(&0x04),
            Some(&MappedKey {
                usage_id: 0x04,
                dom_key: "KeyA",
                prefix: "A",
            })
        );
    }
}
