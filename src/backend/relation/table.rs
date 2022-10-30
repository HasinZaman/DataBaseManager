pub struct Table{
    name: String,
    attributes: Vec<Attribute>,
    primary_id: Option<usize>,
}

pub struct Attribute{
    name: String,
    data_type: AttributeType,
    constraint: Vec<Constraint>
}
pub enum Constraint{
    NotNull,
    Unique,
    ForeignKey{
        table_name: String,
        attribute_name: String
    },
    AutoIncrement,
}
pub enum AttributeType{
    //string data types
    Char(u8),
    VarChar(u16),
    Binary(u8),
    VarBinary(u16),
    TinyBlob,
    TinyText,
    Text(u16),
    Blob(u16),
    MediumText,
    MediumBlob,
    LongText,
    LongBlob,
    Enum{val: Vec<String>},
    Set{val: Vec<AttributeType>},

    //numeric data types
    Bit(u8),
    TinyInt(u8),
    Bool,
    Boolean,
    SmallInt(u8),
    MediumInt(u8),
    Int(u8),
    BigInt(u8),
    Float(u8),
    Decimal(u8, u8),

    //Date time
    Date,
    DateTime,
    TimeStamp,
    Time,
    Year
}