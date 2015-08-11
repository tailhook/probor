use cbor;
use std::error::Error;
use std::fmt::{self, Formatter, Debug, Display};


pub enum DecodeError {
    AbsentField(&'static str),
    UnexpectedNull,
    ExpectationFailed(&'static str, cbor::DecodeError),
    BadFieldValue(&'static str, Box<DecodeError>),
    BadArrayElement(usize, Box<DecodeError>),
    SkippingError(cbor::DecodeError),
}

impl Debug for DecodeError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::DecodeError::*;
        match self {
            &AbsentField(field) => write!(fmt, "absent field {:?}", field),
            &UnexpectedNull => write!(fmt, "null is not expected"),
            &ExpectationFailed(exp, ref err)
            => write!(fmt, "{}: {}", exp, err),
            &BadFieldValue(field, ref err)
            => write!(fmt, "Bad value for {:?}: {}", field, err),
            &BadArrayElement(num, ref err)
            => write!(fmt, "Bad array element {}: {}", num, err),
            &SkippingError(ref err)
            => write!(fmt, "Error when skipping value: {}", err),
        }
    }
}

impl Display for DecodeError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::DecodeError::*;
        match self {
            &AbsentField(field) => write!(fmt, "absent field {:?}", field),
            &UnexpectedNull => write!(fmt, "null is not expected"),
            &ExpectationFailed(exp, ref err)
            => write!(fmt, "{}: {}", exp, err),
            &BadFieldValue(field, ref err)
            => write!(fmt, "Bad value for {:?}: {}", field, err),
            &BadArrayElement(num, ref err)
            => write!(fmt, "Bad array element {}: {}", num, err),
            &SkippingError(ref err)
            => write!(fmt, "Error when skipping value: {}", err),
        }
    }
}

impl Error for DecodeError {
    fn description(&self) -> &'static str {
        use self::DecodeError::*;
        match self {
            &AbsentField(_) => "absent field",
            &UnexpectedNull => "unexpected null",
            &ExpectationFailed(exp, _) => exp,
            &BadFieldValue(_, _) => "bad field value",
            &BadArrayElement(_, _) => "bad array element",
            &SkippingError(_) => "error when skipping value",
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::DecodeError::*;
        match self {
            &AbsentField(_) => None,
            &UnexpectedNull => None,
            &ExpectationFailed(_, ref err) => Some(err),
            &BadFieldValue(_, ref err) => Some(&**err),
            &BadArrayElement(_, ref err) => Some(&**err),
            &SkippingError(ref err) => Some(err),
        }
    }
}