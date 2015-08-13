use cbor::{Decoder, Encoder};
use cbor::decoder::opt;
use {Decodable, Input, DecodeError};
use {Encodable, Output, EncodeError};

impl<A:Decodable> Decodable for Option<A> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        A::decode_opt(e).map(|x| Some(x))
    }
}

impl Decodable for String {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.text())
            .map_err(|e| DecodeError::ExpectationFailed("string expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("u64 expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("u32 expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("u16 expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("u8 expected", e)));
        Ok(res)
    }
}

impl Encodable for u8 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u8(*self)
    }
}

impl Decodable for i64 {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        // TODO(tailhook) implement text iterator
        let res = try!(opt(e.i64())
            .map_err(|e| DecodeError::ExpectationFailed("i64 expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("i32 expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("i16 expected", e)));
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
            .map_err(|e| DecodeError::ExpectationFailed("i8 expected", e)));
        Ok(res)
    }
}

impl Encodable for i8 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.i8(*self)
    }
}
