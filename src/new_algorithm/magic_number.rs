const MAGIC_NUMBER_HEADER_1: u8 = b'F'; // Fm
const MAGIC_NUMBER_HEADER_2: u8 = b'I'; // Index
const MAGIC_NUMBER_VERSION_1: u8 = b'0'; // Major Version
const MAGIC_NUMBER_VERSION_2: u8 = b'0'; // Minor Version

/// Magic number for FM-index (Little-endian)
#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicNumber([u8; 4]);

impl MagicNumber {
    pub fn new() -> Self {
        Self([
            MAGIC_NUMBER_HEADER_1,
            MAGIC_NUMBER_HEADER_2,
            MAGIC_NUMBER_VERSION_1,
            MAGIC_NUMBER_VERSION_2,
        ])
    }

    // Getters
    pub fn major_version(&self) -> u8 {
        self.0[2]
    }
    pub fn minor_version(&self) -> u8 {
        self.0[3]
    }
    // Checkers
    /// Check if the magic number has valid header
    pub fn is_valid(&self) -> bool {
        self.0[0] == MAGIC_NUMBER_HEADER_1
        && self.0[1] == MAGIC_NUMBER_HEADER_2
    }
    /// Check if supported - major & minor version are same
    pub fn is_supported_version(&self, major_version: u8, minor_version: u8) -> bool {
        self.major_version() == major_version
        && self.minor_version() == minor_version
    }
}
