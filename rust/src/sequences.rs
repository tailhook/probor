use std::collections::VecDeque;

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
            .map_err(|e| DecodeError::WrongType("array expected", e)));
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

impl<T:Decodable> Decodable for VecDeque<T> {
    fn decode_opt<R: Input>(e: &mut Decoder<R>)
        -> Result<Option<Self>, DecodeError>
    {
        let num_opt = try!(opt(e.array())
            .map_err(|e| DecodeError::WrongType("array expected", e)));
        if let Some(num) = num_opt {
            let mut res = VecDeque::new();
            for idx in 0..num {
                res.push_back(try!(Decode::decode_elem(e, idx)));
            }
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl<T:Encodable> Encodable for VecDeque<T> {
    fn encode<W: Output>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError>
    {
        try!(e.array(self.len()));
        for i in self {
            try!(i.encode(e));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use std::io::Cursor;
    use {Encodable, Decodable, Config, Encoder, Decoder, decode};

    fn roundtrip<A:Encodable, B:Decodable>(v: A) -> B {
        let mut e = Encoder::new(Vec::new());
        v.encode(&mut e).unwrap();
        let v = e.into_writer();
        println!("Data {:?} {:?}", String::from_utf8_lossy(&v), v);
        let mut d = &mut Decoder::new(Config::default(), Cursor::new(&v[..]));
        decode(d).unwrap()
    }

    #[test]
    fn vec() {
        assert_eq!(roundtrip::<Vec<u32>, Vec<u64>>(vec![0, 1]), vec![0, 1]);
    }

    #[test]
    fn deque() {
        let dq = vec![0, 1].into_iter().collect::<VecDeque<i32>>();
        assert_eq!(roundtrip::<VecDeque<i32>, VecDeque<i32>>(dq.clone()), dq);
    }

    // Types are compatible

    #[test]
    fn into_vec() {
        let dq = vec![0, 1].into_iter().collect::<VecDeque<_>>();
        assert_eq!(roundtrip::<VecDeque<u32>, Vec<u16>>(dq), vec![0, 1]);
    }

    #[test]
    fn into_deque() {
        let dq = vec![0, 1].into_iter().collect::<VecDeque<_>>();
        assert_eq!(roundtrip::<Vec<u64>, VecDeque<u32>>(vec![0, 1]), dq);
    }

}
