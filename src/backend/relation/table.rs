use std::{fmt, collections::HashSet};
use core::hash::Hash;

/// Table struct defines the structure of a relation table
#[derive(Clone, Debug)]
pub struct Table{
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub primary_key: Option<usize>,
}

/// Attribute defines the columns of a Table
#[derive(Clone, Debug)]
pub struct Attribute{
    pub name: String,
    pub data_type: AttributeType,
    pub constraint: HashSet<Constraint>
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.data_type)
    }
}

/// Constraint defines the restrictions of an attribute
#[derive(Clone, Hash, Eq, Debug)]
pub enum Constraint{
    NotNull,
    Unique,
    ForeignKey{
        table_name: String,
        attribute_name: String
    },
    AutoIncrement,
}
impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // (
            //     Self::ForeignKey { .. },
            //     Self::ForeignKey { .. }
            // ) => true,
            
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl fmt::Display for Constraint{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::NotNull => write!(f, "Not Null"),
            Constraint::Unique => write!(f, "Unique"),
            Constraint::ForeignKey{table_name: table,attribute_name: attr} => write!(f, "FOREIGN KEY REFERENCES {}({})", table, attr),
            Constraint::AutoIncrement => write!(f, "Auto Increment"),
        }
    }
}

/// AttributeType defines every type of MySQL datatype
#[derive(Clone, Debug)]
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
impl fmt::Display for AttributeType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //string data types
            AttributeType::Char(val) => write!(f, "char({})", val),
            AttributeType::VarChar(val) => write!(f, "varchar({})", val),
            AttributeType::Binary(val) => write!(f, "binary({})", val),
            AttributeType::VarBinary(val) => write!(f, "varbinary({})", val),
            AttributeType::TinyBlob => write!(f, "tinyblob"),
            AttributeType::TinyText => write!(f, "tinytext"),
            AttributeType::Text(val) => write!(f, "text({})", val),
            AttributeType::Blob(val) => write!(f, "blob({})", val),
            AttributeType::MediumText => write!(f, "mediumtext"),
            AttributeType::MediumBlob => write!(f, "mediumblob"),
            AttributeType::LongText => write!(f, "longtext"),
            AttributeType::LongBlob => write!(f, "longblob"),
            AttributeType::Enum{..} => panic!(),
            AttributeType::Set{..} => todo!(),

            //numeric data types
            AttributeType::Bit(val) => write!(f, "bit({})", val),
            AttributeType::TinyInt(val) => write!(f, "tinyint({})", val),
            AttributeType::Bool => write!(f, "bool"),
            AttributeType::Boolean => write!(f, "boolean"),
            AttributeType::SmallInt(val) => write!(f, "smallint({})", val),
            AttributeType::MediumInt(val) => write!(f, "mediumint({})", val),
            AttributeType::Int(val) => write!(f, "int({})", val),
            AttributeType::BigInt(val) => write!(f, "bigint({})", val),
            AttributeType::Float(val) => write!(f, "float({})", val),
            AttributeType::Decimal(val_1 , val_2) => write!(f, "decmimal({},{})", val_1, val_2),

            //Date time
            AttributeType::Date => write!(f, "date"),
            AttributeType::DateTime => write!(f, "DateTime"),
            AttributeType::TimeStamp => write!(f, "TimeStamp"),
            AttributeType::Time => write!(f, "Time"),
            AttributeType::Year => write!(f, "Year"),
        }
    }
}