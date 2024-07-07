use crate::types::{i32::I32, i64::I64, u32::U32, u64::U64, DataType, Value};

pub fn bytes_to_data(data_type: &DataType, value: &Vec<u8>) -> (Value, String) {
    let printable_value;
    let value = match data_type {
        DataType::I32 => {
            let value = i32::from_le_bytes(
                value
                    .as_slice()
                    .try_into()
                    .expect("Provided value is incorrect"),
            );
            printable_value = value.to_string();
            Value::I32(I32::new(value))
        }
        DataType::I64 => {
            let value = i64::from_le_bytes(
                value
                    .as_slice()
                    .try_into()
                    .expect("Provided value is incorrect"),
            );
            printable_value = value.to_string();
            Value::I64(I64::new(value))
        }
        DataType::U32 => {
            let value = u32::from_le_bytes(
                value
                    .as_slice()
                    .try_into()
                    .expect("Provided value is incorrect"),
            );
            printable_value = value.to_string();
            Value::U32(U32::new(value))
        }
        DataType::U64 => {
            let value = u64::from_le_bytes(
                value
                    .as_slice()
                    .try_into()
                    .expect("Provided value is incorrect"),
            );
            printable_value = value.to_string();
            Value::U64(U64::new(value))
        }
        DataType::Nothing => {
            printable_value = "nothing".to_string();
            Value::Nothing
        }
    };

    (value, printable_value)
}
