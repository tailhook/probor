use cbor::{Decoder, Encoder};
use cbor::decoder::opt;
use {Decodable, Input, DecodeError};
use {Encodable, Output, EncodeError};
use regex::Regex;


impl Decodable for Regex {
    fn decode_opt<R: Input>(d: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let s: Result<Option<String>, _> = Decodable::decode_opt(d);
        match s {
            Ok(Some(x)) => Regex::new(&x)
                .map_err(|_| DecodeError::WrongValue("invalid regexp"))
                .map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Encodable for Regex {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        e.text(self.as_str())
    }
}
