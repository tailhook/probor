use cbor::{Decoder, Encoder};
use cbor::decoder::opt;
use {Decodable, Input, DecodeError};
use {Encodable, Output, EncodeError};
use traits::decode;

impl<T:Decodable> Decodable for Vec<T> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let num_opt = try!(opt(e.array())
            .map_err(|e| DecodeError::ExpectationFailed("array expected", e)));
        if let Some(num) = num_opt {
            let mut res = Vec::new();
            for idx in 0..num {
                res.push(try!(decode(e).map_err(|e|
                    DecodeError::BadArrayElement(idx, Box::new(e)))));
            }
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl<T:Encodable> Encodable for Vec<T> {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        try!(e.array(self.len()));
        for i in self {
            try!(i.encode(e));
        }
        Ok(())
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
            .map_err(|e| DecodeError::ExpectationFailed("string expected", e)));
        Ok(res)
    }
}

impl Encodable for u64 {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.u64(*self)
    }
}
