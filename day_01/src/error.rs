use super::*;
use failure::Fail;
use std::{cmp::Ordering,
          ffi::OsString,
          fmt,
          num::TryFromIntError,
          option::NoneError,};

#[allow(pub_enum_variant_names)]    // unfortunately named redundantly (`NoneError`) in std::
#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    InvalidUtf8Arg(OsString),
    NoneError,
    InvalidDigit(char),
    DigitConversion(TryFromIntErrorPartialEq),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
            Error::NoneError => MSG_ERR_NONE_ERROR.to_owned(),
            Error::InvalidDigit(c) => format!("{}: {}", MSG_ERR_INVALID_DIGIT.to_owned(), c),
            Error::DigitConversion(ref e) => format!("{}: {}", MSG_ERR_DIGIT_CONVERSION.to_owned(), e),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Self {
        Error::DigitConversion(TryFromIntErrorPartialEq(err))
    }
}

#[derive(Clone, Debug, Fail)]
pub struct TryFromIntErrorPartialEq(TryFromIntError);

impl fmt::Display for TryFromIntErrorPartialEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}", self.0))
    }
}

impl PartialEq for TryFromIntErrorPartialEq {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl PartialOrd for TryFromIntErrorPartialEq {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        None
    }
}

