use cbor;
use std::error::Error;
use std::fmt::{self, Formatter, Debug, Display};


pub enum DecodeError {
    AbsentField(&'static str),
    WrongArrayLength(usize),
    DuplicateKey,
    UnexpectedNull,
    WrongType(&'static str, cbor::DecodeError),
    WrongValue(&'static str),
    BadFieldValue(&'static str, Box<DecodeError>),
    BadArrayElement(usize, Box<DecodeError>),
    SkippingError(cbor::DecodeError),
}

impl Debug for DecodeError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::DecodeError::*;
        match self {
            &AbsentField(field) => write!(fmt, "absent field {:?}", field),
            &WrongArrayLength(n) => write!(fmt, "wrong array length {:?}", n),
            &DuplicateKey => write!(fmt, "some key is duplicated"),
            &UnexpectedNull => write!(fmt, "null is not expected"),
            &WrongType(exp, ref err) => write!(fmt, "{}: {}", exp, err),
            &WrongValue(exp) => write!(fmt, "{}", exp),
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
            &WrongArrayLength(n) => write!(fmt, "wrong array length {:?}", n),
            &DuplicateKey => write!(fmt, "some key is duplicated"),
            &UnexpectedNull => write!(fmt, "null is not expected"),
            &WrongType(exp, ref err) => write!(fmt, "{}: {}", exp, err),
            &WrongValue(exp) => write!(fmt, "{}", exp),
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
            &WrongArrayLength(_) => "wrong array length",
            &DuplicateKey => "some key is duplicated",
            &UnexpectedNull => "unexpected null",
            &WrongType(exp, _) => exp,
            &WrongValue(exp) => exp,
            &BadFieldValue(_, _) => "bad field value",
            &BadArrayElement(_, _) => "bad array element",
            &SkippingError(_) => "error when skipping value",
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::DecodeError::*;
        match self {
            &AbsentField(_) => None,
            &WrongArrayLength(_) => None,
            &DuplicateKey => None,
            &UnexpectedNull => None,
            &WrongType(_, ref err) => Some(err),
            &WrongValue(_) => None,
            &BadFieldValue(_, ref err) => Some(&**err),
            &BadArrayElement(_, ref err) => Some(&**err),
            &SkippingError(ref err) => Some(err),
        }
    }
}

