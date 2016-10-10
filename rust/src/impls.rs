use std::sync::Arc;

use cbor::{Decoder, Encoder};
use cbor::decoder::opt;
use {Decodable, Input, DecodeError};
use {Encodable, Output, EncodeError};

impl<A:Decodable> Decodable for Option<A> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        A::decode_opt(e).map(Some)
    }
}

impl<A:Encodable> Encodable for Option<A> {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        match self {
            &Some(ref x) => x.encode(e),
            &None => e.null(),
        }
    }
}

impl<A:Decodable> Decodable for Box<A> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        A::decode_opt(e).map(|x| x.map(Box::new))
    }
}

impl<A:Encodable> Encodable for Box<A> {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        (**self).encode(e)
    }
}

impl<A:Decodable> Decodable for Arc<A> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        A::decode_opt(e).map(|x| x.map(Arc::new))
    }
}

impl<A:Encodable> Encodable for Arc<A> {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        (**self).encode(e)
    }
}

impl<'x, T:Encodable> Encodable for &'x T {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        (**self).encode(e)
    }
}

impl Decodable for String {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.text())
            .map_err(|e| DecodeError::WrongType("string expected", e)));
        Ok(res)
    }
}

impl Encodable for String {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.text(self)
    }
}

impl Decodable for u64 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.u64())
            .map_err(|e| DecodeError::WrongType("u64 expected", e)));
        Ok(res)
    }
}

impl Encodable for u64 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u64(*self)
    }
}

impl Decodable for u32 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.u32())
            .map_err(|e| DecodeError::WrongType("u32 expected", e)));
        Ok(res)
    }
}

impl Encodable for u32 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u32(*self)
    }
}
impl Decodable for u16 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.u16())
            .map_err(|e| DecodeError::WrongType("u16 expected", e)));
        Ok(res)
    }
}

impl Encodable for u16 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u16(*self)
    }
}

impl Decodable for u8 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.u8())
            .map_err(|e| DecodeError::WrongType("u8 expected", e)));
        Ok(res)
    }
}

impl Encodable for u8 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u8(*self)
    }
}

impl Decodable for bool {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.bool())
            .map_err(|e| DecodeError::WrongType("bool expected", e)));
        Ok(res)
    }
}

impl Encodable for bool {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.bool(*self)
    }
}

impl Decodable for i64 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.i64())
            .map_err(|e| DecodeError::WrongType("i64 expected", e)));
        Ok(res)
    }
}

impl Encodable for i64 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.i64(*self)
    }
}

impl Decodable for i32 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.i32())
            .map_err(|e| DecodeError::WrongType("i32 expected", e)));
        Ok(res)
    }
}

impl Encodable for i32 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.i32(*self)
    }
}
impl Decodable for i16 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.i16())
            .map_err(|e| DecodeError::WrongType("i16 expected", e)));
        Ok(res)
    }
}

impl Encodable for i16 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.i16(*self)
    }
}

impl Decodable for i8 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.i8())
            .map_err(|e| DecodeError::WrongType("i8 expected", e)));
        Ok(res)
    }
}

impl Encodable for i8 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.i8(*self)
    }
}

impl Decodable for f64 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.f64())
            .map_err(|e| DecodeError::WrongType("i8 expected", e)));
        Ok(res)
    }
}

impl Encodable for f64 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.f64(*self)
    }
}

impl Decodable for f32 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.f32())
            .map_err(|e| DecodeError::WrongType("i8 expected", e)));
        Ok(res)
    }
}

impl Encodable for f32 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.f32(*self)
    }
}

#[cfg(target_pointer_width="64")]
impl Decodable for usize {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.u64().map(|x| x as usize))
            .map_err(|e| DecodeError::WrongType("usize expected", e)));
        Ok(res)
    }
}

#[cfg(target_pointer_width="64")]
impl Encodable for usize {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u64(*self as u64)
    }
}

#[cfg(target_pointer_width="32")]
impl Decodable for usize {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.u32().map(|x| x as usize))
            .map_err(|e| DecodeError::WrongType("usize expected", e)));
        Ok(res)
    }
}

#[cfg(target_pointer_width="32")]
impl Encodable for usize {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u32(*self as u32)
    }
}
