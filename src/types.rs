#[derive(Debug)]
pub enum Value {
    I64(I64),
}

#[derive(Debug)]
pub struct I64 {
    pub value: i64,
}
impl I64 {
    pub fn new(value: i64) -> I64 {
        I64 { value }
    }
}
