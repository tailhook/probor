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

pub trait Decodable {
    /// Decode an object or null
    ///
    /// This must be optional return so that any value may become optional and
    /// we can't determine it in advance
    fn decode_opt<R: Input>(d: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>;
}

/// Decodes and object and asserts that it's not null
pub fn decode<T:Decodable, R:Input>(d: &mut Decoder<R>)
    -> Result<T, DecodeError>
{
    match T::decode_opt(d) {
        Ok(Some(x)) => Ok(x),
        Ok(None) => Err(DecodeError::UnexpectedNull),
        Err(e) => Err(e)
    }
}
