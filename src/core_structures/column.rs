#[derive(Debug, Clone)]
pub enum ColumnType {
    // Text Types
    String(u64),
    JSON(u64),

    // Byte Types
    Bool(u8),
    // Bytea,

    // Numeric Types
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    SignedBigInteger(i64),
    BigInteger(u64),

    // Identifier Types
    Snowflake(u64),
    UUID,

    // Color Types
    RGB,
    RGBA,
    CYMK,
    PMS, // Pantone
    Pantone, // PMS
    Hex,

    // Boundless Types
    Array(Box<ColumnType>, u64),
}