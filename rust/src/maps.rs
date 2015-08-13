use std::hash::Hash;
use std::collections::HashMap;

use cbor::decoder::opt;
use {Decoder, Decodable, Input, DecodeError};
use {Encoder, Encodable, Output, EncodeError};
use traits::Decode;


impl<K:Decodable+Hash+Eq, V:Decodable> Decodable for HashMap<K, V> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let num_opt = try!(opt(e.object())
            .map_err(|e| DecodeError::ExpectationFailed("object expected", e)));
        if let Some(num) = num_opt {
            let mut res = HashMap::new();
            for _ in 0..num {
                // is decode_field is good enough?
                let k = try!(Decode::decode_field(e, "mapping key"));
                let v = try!(Decode::decode_field(e, "mapping value"));
                match res.insert(k, v) {
                    Some(_) => return Err(DecodeError::DuplicateKey),
                    None => {}
                }
            }
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl<K:Encodable+Hash+Eq, V:Encodable> Encodable for HashMap<K, V> {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        try!(e.object(self.len()));
        for (k, v) in self {
            try!(k.encode(e));
            try!(v.encode(e));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::io::Cursor;
    use {Encodable, Decodable, Config, Encoder, Decoder, decode};

    fn roundtrip<A:Encodable, B:Decodable>(v: &A) -> B {
        let mut e = Encoder::new(Vec::new());
        v.encode(&mut e).unwrap();
        let v = e.into_writer();
        println!("Data {:?} {:?}", String::from_utf8_lossy(&v), v);
        let mut d = &mut Decoder::new(Config::default(), Cursor::new(&v[..]));
        decode(d).unwrap()
    }

    #[test]
    fn map() {
        let map: HashMap<_, _> = vec![
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ].into_iter().collect();
        assert_eq!(roundtrip::<HashMap<String, i32>,
                               HashMap<String, i32>>(&map), map);
    }

}
