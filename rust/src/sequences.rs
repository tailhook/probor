use cbor::{Decoder, Encoder};
use cbor::decoder::opt;
use {Decodable, Input, DecodeError};
use {Encodable, Output, EncodeError};
use traits::Decode;


impl<T:Decodable> Decodable for Vec<T> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let num_opt = try!(opt(e.array())
            .map_err(|e| DecodeError::ExpectationFailed("array expected", e)));
        if let Some(num) = num_opt {
            let mut res = Vec::new();
            for idx in 0..num {
                res.push(try!(Decode::decode_elem(e, idx)));
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
