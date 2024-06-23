pub mod i32;
pub mod i64;
pub mod u32;
pub mod u64;

use i32::I32;
use i64::I64;
use u32::U32;
use u64::U64;

#[derive(Debug)]
pub enum Value {
    I32(I32),
    I64(I64),
    U32(U32),
    U64(U64),
    Nothing,
}

impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Value::I32(_) => DataType::I32,
            Value::I64(_) => DataType::I64,
            Value::U32(_) => DataType::U32,
            Value::U64(_) => DataType::U64,
            Value::Nothing => DataType::Nothing,
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    I32,
    I64,
    U32,
    U64,
    Nothing,
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::I32, DataType::I32) => true,
            (DataType::I64, DataType::I64) => true,
            (DataType::U32, DataType::U32) => true,
            (DataType::U64, DataType::U64) => true,
            (DataType::Nothing, DataType::Nothing) => true,
            _ => false,
        }
    }
}
