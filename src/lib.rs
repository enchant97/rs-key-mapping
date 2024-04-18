include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Keyboard layouts, used to convert between key-code types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
