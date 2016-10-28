use std::{error,io};

pub trait Instruction: Sized {
    type DecodeError: error::Error;

    fn mnemonic(&self) -> &'static str;
    fn decode<R>(reader: R) -> Result<Self, Self::DecodeError> where R: io::Read;
}
