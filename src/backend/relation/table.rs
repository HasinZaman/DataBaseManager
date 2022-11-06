#[derive(Clone)]
pub struct Table{
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary_key: Option<usize>,
}

#[derive(Clone)]
pub struct Attribute{
    pub name: String,
    pub data_type: AttributeType,
    pub constraint: Vec<Constraint>
}


#[derive(Clone)]
pub enum Constraint{
    NotNull,
    Unique,
    ForeignKey{
        table_name: String,
        attribute_name: String
    },
    AutoIncrement,
}

#[derive(Clone)]
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