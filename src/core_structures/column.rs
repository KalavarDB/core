/// Bit count definitions, used in storage calculations
#[allow(dead_code)]
const BIT: u64 = 1;
#[allow(dead_code)]

const BYTE: u64 = 8 * BIT;
#[allow(dead_code)]

const KB: u64 = 1024 * BYTE;
#[allow(dead_code)]

const MB: u64 = 1024 * KB;
#[allow(dead_code)]

const GB: u64 = 1024 * MB;
#[allow(dead_code)]

/// An enumerator which defines the available column types
/// I recommend reading the documentation at the link below for details on them all:
/// <https://kalavar.cf/documentation/data-types/>
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ColumnTypeEnum {
    // Text Types
    /// A generic String type
    String,
    /// JSON flavoured string
    JSON,

    // Byte Types
    /// Boolean data type for columns
    Bool,
    /// 8 Bit storage type
    Byte,
    /// Binary Large Object type
    BLOB,

    // Numeric Types
    /// Unsigned 8 bit integer
    Integer8,
    /// Unsigned 16 bit integer
    Integer16,
    /// Unsigned 32 bit integee
    Integer32,
    /// Unsigned 64 bit integer
    Integer64,
    /// Signed 8 bit integer
    SignedInteger8,
    /// Signed 16 bit integer
    SignedInteger16,
    /// Signed 32 bit integer
    SignedInteger32,
    /// Signed 64 bit integer
    SignedInteger64,
    /// An unsigned integer, comprised of other integers to provide maximum storage capacity
    BigInteger,
    /// A signed integer, comprised of other integers to provide maximum storage capacity
    SignedBigInteger,

    // Identifier Types
    /// Snowflake unique identifier
    Snowflake,
    /// Universally Unique Identifier
    UUID,
    /// Sony's re-thought version of Snowflake
    SonyFlake,

    // Color Types
    /// Red, Green, Blue color type
    RGB,
    /// Red, Green, Blue, Alpha color type
    RGBA,
    /// Cyan, Magenta, Yellow, Black color type
    CMYK,
    /// Hexadecimal color type
    Hex,

    // Boundless Types
    /// An array type for the column
    Array,
    /// An enumerator type for the column, see the [docs](https://kalavar.cf/documentation/data-types/enum/) for more info
    Enum,

    // Network Types
    /// Simple validated IPv4 address type
    IPv4,
    /// Simple validated IPv6 address type
    IPv6,
    /// Simple MAC address type
    Mac,
    /// Mac8, currently unknown but PostgreSQL has it so it must be needed by *someone*
    Mac8,

    // Timestamps
    /// A timestamp, prefixed with information about the timezone it is for to allow for conversion
    Timestamp,
    /// A timestamp without the prefixed timezone data, assumes it is UTC and converts to `Timetstamp` as though it was UTC based, even if it may not be
    NaiveTimestamp,
}

/// A structure designed to incorporate the data type it stores, as well as the minimum and maximum byte count of the type
#[derive(Debug, Clone)]
pub struct ColumnType {
    /// If the column is to be hidden in results from queries
    pub is_private: bool,
    /// The type of the column
    pub inner_type: ColumnTypeEnum,
    /// The minimum length of the column type in question (the value this represents actually changes based on the inner_type
    pub min_len: u64,
    /// The maximum length of the column type in question (the value this represents actually changes based on the inner_type
    pub max_len: u128,
    /// The potential sub type of the type, if applicable
    pub value_type: Option<ColumnTypeEnum>,
}

impl ColumnType {
    /// Helper method to generate a new column
    #[allow(dead_code)]
    pub fn new(
        inner: ColumnTypeEnum,
        value_type: Option<ColumnTypeEnum>,
        length: u64,
    ) -> ColumnType {
        // Build a default column entry
        let t = ColumnType {
            is_private: false,
            inner_type: inner.clone(),
            min_len: 0,
            max_len: 0,
            value_type,
        };

        // Set the values of that entry, as well as returning it to the caller automatically
        return process_column(t, inner, length);
    }

    /// Helper method to generate a new private column which remains hidden from query results
    #[allow(dead_code)]
    pub fn new_prv(
        inner: ColumnTypeEnum,
        value_type: Option<ColumnTypeEnum>,
        length: u64,
    ) -> ColumnType {
        // Build a default column entry
        let t = ColumnType {
            is_private: true,
            inner_type: inner.clone(),
            min_len: 0,
            max_len: 0,
            value_type,
        };

        // Set the values of that entry, as well as returning it to the caller automatically
        return process_column(t, inner, length);
    }
}

/// Helper function for serialization using the record types
pub fn process_column(mut t: ColumnType, inner: ColumnTypeEnum, length: u64) -> ColumnType {
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
            t.min_len = (BYTE * 4) + 3;
            t.max_len = ((BYTE * 4) + 3) as u128;
            t
        }
        ColumnTypeEnum::IPv6 => {
            t.min_len = (BYTE * 16) + 7;
            t.max_len = ((BYTE * 16) + 7) as u128;
            t
        }
        ColumnTypeEnum::Mac => {
            t.min_len = BIT * 48;
            t.max_len = (BIT * 48) as u128;
            t
        }
        ColumnTypeEnum::Mac8 => {
            // TODO: check bit length of MAC8 addresses
            t.min_len = BIT * 48;
            t.max_len = (BIT * 48) as u128;
            t
        }
        ColumnTypeEnum::Timestamp => {
            t.min_len = BYTE * 10;
            t.max_len = (BYTE * 10) as u128;
            t
        }
        ColumnTypeEnum::NaiveTimestamp => {
            t.min_len = BYTE * 9;
            t.max_len = (BYTE * 9) as u128;
            t
        }
    };
}
