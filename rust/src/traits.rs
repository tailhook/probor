use cbor::{Encoder};
use cbor::{Decoder};
use cbor::skip::{Skip};
use byteorder::{WriteBytesExt, ReadBytesExt};

use {EncodeError, DecodeError};

pub trait Output: WriteBytesExt {}
pub trait Input: ReadBytesExt + Skip {}

impl<T:WriteBytesExt> Output for T {}
impl<T:ReadBytesExt + Skip> Input for T {}


pub trait Encodable {
    fn encode<W: Output>(&self, e: &mut Encoder<W>)
        -> Result<(), EncodeError>;
}

pub trait Decodable: Sized {
    /// Decode an object or null
    ///
    /// This must be optional return so that any value may become optional and
    /// we can't determine it in advance
    fn decode_opt<R: Input>(d: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>;
}

pub trait Decode: Sized {
    /// Decodes non-null array element and correctly propagates error.
    ///
    /// The `n` is used only for errors
    ///
    /// Propagating errors with try!(decode()) is dangerous as it may propagate
    /// UnexpectedNull in the middle of an array, which user may think it's
    /// just null. try!(decode_elem()) is safe
    fn decode_elem<R:Input>(d: &mut Decoder<R>, n: usize)
        -> Result<Self, DecodeError>;

    /// Decodes non-null field and correctly propagates error.
    ///
    /// The `name` is used only for errors
    ///
    /// Propagating errors with try!(decode()) is dangerous as it may propagate
    /// UnexpectedNull in the middle of an array, which user may think it's
    /// just null. try!(decode_field()) is safe
    fn decode_field<R:Input>(d: &mut Decoder<R>, name: &'static str)
        -> Result<Self, DecodeError>;
}

impl<T:Decodable> Decode for T {

    fn decode_elem<R:Input>(d: &mut Decoder<R>, n: usize)
        -> Result<Self, DecodeError>
    {
        match T::decode_opt(d) {
            Ok(Some(x)) => Ok(x),
            Ok(None) => Err(DecodeError::BadArrayElement(n,
                Box::new(DecodeError::UnexpectedNull))),
            Err(e) => Err(DecodeError::BadArrayElement(n, Box::new(e))),
        }
    }

    fn decode_field<R:Input>(d: &mut Decoder<R>, name: &'static str)
        -> Result<Self, DecodeError>
    {
        match T::decode_opt(d) {
            Ok(Some(x)) => Ok(x),
            Ok(None) => Err(DecodeError::BadFieldValue(name,
                Box::new(DecodeError::UnexpectedNull))),
            Err(e) => Err(DecodeError::BadFieldValue(name, Box::new(e))),
        }
    }
}

/// Decodes and object and asserts that it's not null
///
/// Use only as top-level decode function. For sub-items use
/// Decode::decode_elem/decode_field
pub fn decode<R:Input, T:Decodable>(d: &mut Decoder<R>) -> Result<T, DecodeError>
{
    match T::decode_opt(d) {
        Ok(Some(x)) => Ok(x),
        Ok(None) => Err(DecodeError::UnexpectedNull),
        Err(e) => Err(e)
    }
}
