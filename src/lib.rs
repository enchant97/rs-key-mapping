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
    pub fn key_code_to_usage_id(&self, key_code: &str) -> Option<&u8> {
        match self {
            Self::US => KEY_CODES_US.get(key_code),
            Self::UK => KEY_CODES_UK.get(key_code),
        }
    }
}
