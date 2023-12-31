use parse_tool::{CheckError, InputBuf};

pub struct MB {
    pub age: u8,
    pub ty: u16,
}

pub trait Decode<I: InputBuf, O, E> {
    fn decode(input: &mut I) -> Result<O, E>;
}

impl<I: InputBuf> Decode<I, (), CheckError> for u8 {
    fn decode(input: &mut I) -> Result<(), CheckError> {
        input.read_u8_ne()?;
        Ok(())
    }
}
impl<I: InputBuf> Decode<I, Self, CheckError> for u8 {
    fn decode(input: &mut I) -> Result<Self, CheckError> {
        input.read_u8_ne()
    }
}

fn main() {
    let arr = [1u8; 1];
    let s: u8 = u8::decode(&mut (&arr[..], ())).unwrap();
    dbg!(s);
}
