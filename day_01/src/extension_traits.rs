use std::convert::TryFrom;
use super::{Error, Result};

pub trait ToDigitOrError {
    fn to_digit_or_error(&self, base: u32) -> Result<u8>;
}

impl ToDigitOrError for char {
    fn to_digit_or_error(&self, base: u32) -> Result<u8> {
        match self.to_digit(base) {
            Some(d) => match u8::try_from(d) {
                Ok(v) => Ok(v),
                Err(e) => Err(e)?,
            },
            None => Err(Error::InvalidDigit(*self)),
        }
    }
}
