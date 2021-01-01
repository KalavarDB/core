#[derive(Debug, Clone)]
pub enum ColumnType {
    // Text Types
    String(u64),
    JSON(u64),

    // Byte Types
    Bool(u8),
    Byte,
    BLOB(u64),

    // Numeric Types
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    BigInteger(u64),
    SignedBigInteger(i64),

    // Identifier Types
    Snowflake(u64),
    UUID,
    SonyFlake(u64),

    // Color Types
    RGB,
    RGBA,
    CMYK,
    Hex,

    // Boundless Types
    Array(Box<ColumnType>, u64),
    Enum(String),

    // Network Types
    IPv4,
    IPv6,
    Mac,
    Mac8,

    // Timestamps
    Timestamp,
    NaiveTimestamp
}

