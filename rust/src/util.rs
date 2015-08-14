use std::io::Cursor;
use {Encodable, Decodable, Encoder, Decoder, DecodeError, Config, decode};


pub fn to_buf<E:Encodable>(val: &E) -> Vec<u8> {
    let mut enc = Encoder::new(Vec::new());
    // We assume that encoding can't fail writing into Vec
    val.encode(&mut enc).unwrap();
    return enc.into_writer();
}

pub fn from_slice<D:Decodable, S:AsRef<[u8]>>(src: S) -> Result<D, DecodeError>
{
    let src = src.as_ref();
    let cur = Cursor::new(src);
    let mut dec = Decoder::new(Config::default(), cur);
    let val = decode(&mut dec);
    if val.is_ok() {
        if dec.into_reader().position() != src.len() as u64 {
            return Err(DecodeError::WrongValue("trailing data"));
        }
    }
    val
}
