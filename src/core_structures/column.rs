// Bit count definitions, used in storage calculations
const BIT: u64 = 1;
const BYTE: u64 = 8 * BIT;
const KB: u64 = 1024 * BYTE;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;

// An enumerator which defines the available column types
// I recommend reading the documentation at the link below for details on them all:
// https://kalavar.cf/documentation/data-types/
#[derive(Debug, Clone)]
pub enum ColumnTypeEnum {
    // Text Types
    String,
    JSON,

    // Byte Types
    Bool,
    Byte,
    BLOB,

    // Numeric Types
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    BigInteger,
    SignedBigInteger,

    // Identifier Types
    Snowflake,
    UUID,
    SonyFlake,

    // Color Types
    RGB,
    RGBA,
    CMYK,
    Hex,

    // Boundless Types
    Array,
    Enum,

    // Network Types
    IPv4,
    IPv6,
    Mac,
    Mac8,

    // Timestamps
    Timestamp,
    NaiveTimestamp,
}

// A structure designed to incorporate the data type it stores, as well as the minimum and maximum byte count of the type
#[derive(Debug, Clone)]
pub struct ColumnType {
    inner_type: ColumnTypeEnum,
    min_len: u64,
    max_len: u128,
    value_type: Option<ColumnTypeEnum>,
}


impl ColumnType {
    // Helper method to generate a new column
    pub fn new(inner: ColumnTypeEnum, value_type: Option<ColumnTypeEnum>, length: u64) -> ColumnType {
        // Build a default column entry
        let mut t = ColumnType {
            inner_type: inner.clone(),
            min_len: 0,
            max_len: 0,
            value_type,
        };

        // Set the values of that entry, as well as returning it to the caller automatically
        return match inner {
            ColumnTypeEnum::String | ColumnTypeEnum::JSON | ColumnTypeEnum::BLOB => {
                t.min_len = BYTE;
                t.max_len = u128::max_value();
                t
            }
            ColumnTypeEnum::Bool => {
                t.min_len = BIT;
                t.max_len = BIT as u128;
                t
            }
            ColumnTypeEnum::Byte => {
                t.min_len = BYTE;
                t.max_len = BYTE as u128;
                t
            }
            ColumnTypeEnum::SignedInteger8 | ColumnTypeEnum::Integer8 => {
                t.min_len = BYTE;
                t.max_len = BYTE as u128;
                t
            }
            ColumnTypeEnum::SignedInteger16 | ColumnTypeEnum::Integer16 => {
                t.min_len = BYTE * 2;
                t.max_len = (BYTE * 2) as u128;
                t
            }
            ColumnTypeEnum::SignedInteger32 | ColumnTypeEnum::Integer32 => {
                t.min_len = BYTE * 4;
                t.max_len = (BYTE * 4) as u128;
                t
            }
            ColumnTypeEnum::SignedInteger64 | ColumnTypeEnum::Integer64 => {
                t.min_len = BYTE * BYTE;
                t.max_len = (BYTE * BYTE) as u128;
                t
            }
            ColumnTypeEnum::SignedBigInteger | ColumnTypeEnum::BigInteger => {
                t.min_len = BYTE;
                t.max_len = KB as u128;
                t
            }
            ColumnTypeEnum::Snowflake => {
                t.min_len = BYTE * BYTE;
                t.max_len = (BYTE * BYTE) as u128;
                t
            }
            ColumnTypeEnum::UUID => {
                t.min_len = BYTE * 32;
                t.max_len = (BYTE * 32) as u128;
                t
            }
            ColumnTypeEnum::SonyFlake => {
                t.min_len = BYTE * BYTE;
                t.max_len = (BYTE * BYTE) as u128;
                t
            }
            ColumnTypeEnum::RGB => {
                t.min_len = BYTE * 6;
                t.max_len = (BYTE * 6) as u128;
                t
            }
            ColumnTypeEnum::RGBA => {
                t.min_len = BYTE * 7;
                t.max_len = (BYTE * 7) as u128;
                t
            }
            ColumnTypeEnum::CMYK => {
                t.min_len = BYTE * 7;
                t.max_len = (BYTE * 7) as u128;
                t
            }
            ColumnTypeEnum::Hex => {
                t.min_len = BYTE * 3;
                t.max_len = (BYTE * 7) as u128;
                t
            }
            ColumnTypeEnum::Array => {
                t.min_len = BYTE;
                t.max_len = (BYTE * length) as u128;
                t
            }
            ColumnTypeEnum::Enum => {
                t.min_len = BYTE;
                t.max_len = (BYTE * 4) as u128;
                t
            }
            ColumnTypeEnum::IPv4 => {
                t.min_len = (BYTE*4)+3;
                t.max_len = ((BYTE*4)+3) as u128;
                t
            }
            ColumnTypeEnum::IPv6 => {
                t.min_len = (BYTE*16)+7;
                t.max_len = ((BYTE*16)+7) as u128;
                t
            }
            ColumnTypeEnum::Mac => {
                t.min_len = (BIT*48);
                t.max_len = (BIT*48) as u128;
                t
            }
            ColumnTypeEnum::Mac8 => {
                // TODO: check bit length of MAC8 addresses
                t.min_len = (BIT*48);
                t.max_len = (BIT*48) as u128;
                t
            }
            ColumnTypeEnum::Timestamp => {
                t.min_len = (BYTE*10);
                t.max_len = (BYTE*10) as u128;
                t
            }
            ColumnTypeEnum::NaiveTimestamp => {
                t.min_len = (BYTE*9);
                t.max_len = (BYTE*9) as u128;
                t
            }
        };
    }
}