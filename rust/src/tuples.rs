use cbor::decoder::opt;
use {Encodable, Encoder, Output, EncodeError};
use {Decodable, Decoder, Input, DecodeError, Decode};

impl<A:Encodable> Encodable for (A,) {
    fn encode<W:Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError> {
        try!(e.array(1));
        try!(self.0.encode(e));
        Ok(())
    }
}

impl<A:Encodable, B:Encodable> Encodable for (A, B) {
    fn encode<W:Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError> {
        try!(e.array(2));
        try!(self.0.encode(e));
        try!(self.1.encode(e));
        Ok(())
    }
}

impl<A:Encodable, B:Encodable, C:Encodable> Encodable for (A, B, C) {
    fn encode<W:Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError> {
        try!(e.array(3));
        try!(self.0.encode(e));
        try!(self.1.encode(e));
        try!(self.2.encode(e));
        Ok(())
    }
}

impl<A:Decodable> Decodable for (A,) {
    fn decode_opt<R:Input>(d: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let len = match opt(d.array()) {
            Ok(Some(x)) => x,
            Ok(None) => return Ok(None),
            Err(e) => return Err(DecodeError::WrongType(
                "expected an array of one element", e)),
        };
        if len != 1 {
            return Err(DecodeError::WrongArrayLength(len));
        }
        Ok(Some((try!(Decode::decode_elem(d, 0)), )))
    }
}

impl<A:Decodable, B:Decodable> Decodable for (A, B) {
    fn decode_opt<R:Input>(d: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let len = match opt(d.array()) {
            Ok(Some(x)) => x,
            Ok(None) => return Ok(None),
            Err(e) => return Err(DecodeError::WrongType(
                "expected an array of two elements", e)),
        };
        if len != 2 {
            return Err(DecodeError::WrongArrayLength(len));
        }
        Ok(Some((try!(Decode::decode_elem(d, 0)),
                 try!(Decode::decode_elem(d, 1)))))
    }
}

impl<A:Decodable, B:Decodable, C:Decodable> Decodable for (A, B, C) {
    fn decode_opt<R:Input>(d: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let len = match opt(d.array()) {
            Ok(Some(x)) => x,
            Ok(None) => return Ok(None),
            Err(e) => return Err(DecodeError::WrongType(
                "expected an array of three elements", e)),
        };
        if len != 3 {
            return Err(DecodeError::WrongArrayLength(len));
        }
        Ok(Some((try!(Decode::decode_elem(d, 0)),
                 try!(Decode::decode_elem(d, 1)),
                 try!(Decode::decode_elem(d, 2)))))
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use {Encodable, Decodable, Config, Encoder, Decoder, decode};

    fn roundtrip<A:Encodable+Decodable>(v: A) -> A {
        let mut e = Encoder::new(Vec::new());
        v.encode(&mut e).unwrap();
        let v = e.into_writer();
        println!("Data {:?} {:?}", String::from_utf8_lossy(&v), v);
        let mut d = &mut Decoder::new(Config::default(), Cursor::new(&v[..]));
        decode(d).unwrap()
    }

    #[test]
    fn one() {
        assert_eq!(roundtrip((1,)), (1,));
    }

    #[test]
    fn two() {
        assert_eq!(roundtrip((21, 22)), (21, 22));
    }

    #[test]
    fn three() {
        assert_eq!(roundtrip((301, 302, 303)), (301, 302, 303));
    }
}
