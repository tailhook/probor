#[macro_export]
macro_rules! _probor_pattern {
    ($x:pat) => { $x }
}
#[macro_export]
macro_rules! _probor_enum_pattern {
    ($name:ident $variant:ident) => {
        $name::$variant
    };
    ($name:ident $variant:ident $($fname:ident)* ) => {
        $name::$variant ( $(ref $fname),*)
    };
}

#[macro_export]
macro_rules! _probor_pattern {
    ($x:pat) => { $x }
}

#[macro_export]
macro_rules! _probor_encode_pos_enum_field {
    ($encoder:expr, $idx:expr, $field:ident, $n:tt) => {
        _probor_skip_to!($encoder, $idx, $n);
        try!($crate::Encodable::encode($field, $encoder));
    };
}

#[macro_export]
macro_rules! _probor_decode_variant {
   ($typ:ident, $dec:expr, $len:expr, $variant:ident) => {{
        // Must read the array so that cbor data is not broken
        // also help backward compatibility of data
        for _ in 1..$len {
            try!($dec.skip().map_err(|e|
                    $crate::DecodeError::SkippingError(e)));
        }
        $typ::$variant
   }};
   ($typ:ident, $dec:expr, $len: expr, $variant:ident,
    $( $fname:ident $fnum:tt ),+) => {{
        $(
            let mut $fname = None;
        )*
        for idx in 1..$len {
            match idx {
                $(
                    _probor_pattern!($fnum) => {
                        // TODO(tailhook) may be we need to support
                        //                decoding Option as well
                        $fname = Some(try!(
                            $crate::Decode::decode_elem($dec, idx)));
                    }
                )*
                _ => try!($dec.skip().map_err(|e|
                    $crate::DecodeError::SkippingError(e))),
            }
        }
        $(
            let $fname = try!($fname.ok_or(
                $crate::DecodeError::AbsentField(
                    stringify!(#$fnum))));
        )*
        $typ::$variant( $( $fname ),* )
    }};
}

#[macro_export]
macro_rules! _probor_decode_enum {
    ($dec:expr, $len:expr, $name:ident { $(
        #$n:tt $variant:ident ( $($fname:ident #$fnum:tt),* ),
    )* }) => {{
        if $len < 1 {
            return Err($crate::DecodeError::WrongValue("array too short"));
        }
        enum Kind {
            $( $variant, )*
        }
        let variant = match $dec.u64() {
            $(
                Ok(_probor_pattern!($n)) => Kind::$variant,
            )*
            Ok(_) => return Err($crate::DecodeError::WrongValue(
                "unknown enum variant")),
            Err($crate::_cbor::DecodeError::UnexpectedType {
                datatype: $crate::_cbor::types::Type::Text, info: inf }) => {
                let txt = try!(String::from_utf8(
                    // TODO(tailhook) use limit from config
                    try!($dec.kernel().raw_data(inf, 1 << 31)
                    .map_err(|e| $crate::DecodeError::WrongType(
                        "int or text expected as enum kind", e))))
                    .map_err(|_| $crate::DecodeError::WrongValue(
                        "enum variant is not correct utf8")));
                match &txt[..] {
                    $(
                        stringify!($variant) => Kind::$variant,
                    )*
                    _ => return Err($crate::DecodeError::WrongValue(
                        "unknown enum variant")),
                }
            }
            Err(e) => {
                return Err($crate::DecodeError::WrongType(
                    "array expected", e));
            }
        };
        match variant {
            $(
                Kind::$variant => {
                    Ok(Some(
                        _probor_decode_variant!($name, $dec, $len, $variant
                            $(, $fname $fnum)*)
                    ))
                }
            )*
        }
    }}
}

#[macro_export]
macro_rules! probor_enum_encoder_decoder {
    ($name:ident { $(
        #$n:tt $variant:ident ( $($fname:ident #$fnum:tt),* ),
    )* }) => {
        impl $crate::Encodable for $name {
            fn encode<W: $crate::Output>(&self,
                e: &mut $crate::_cbor::Encoder<W>)
                -> Result<(), $crate::_cbor::EncodeError>
            {
                match self {
                    $(
                        &_probor_enum_pattern!($name $variant $($fname)* ) => {
                            try!(e.array(_probor_max!( $($fnum,)* ) + 1));
                            let mut idx = 0; // hope this can be optimized out
                            try!(e.u64($n));
                            $(
                                _probor_encode_pos_enum_field!(
                                    e, idx, $fname, $fnum);
                            )*
                            (idx += 1, idx); // silence warnings
                        },
                    )*
                }
                Ok(())
            }
        }
        impl $crate::Decodable for $name {
            fn decode_opt<R: $crate::Input>(
                d: &mut $crate::_cbor::Decoder<R>)
                -> Result<Option<Self>, $crate::DecodeError>
            {
                match d.array() {
                    Ok(array_len) => {
                        _probor_decode_enum!(d, array_len, $name { $(
                            #$n $variant ( $($fname #$fnum),* ),
                        )* })
                    }
                    Err($crate::_cbor::DecodeError::UnexpectedType {
                        datatype: $crate::_cbor::types::Type::Null, .. }) => {
                        return Ok(None);
                    }
                    Err(e) => {
                        return Err($crate::DecodeError::WrongType(
                            "array or object expected (e5)", e));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test_enum {
    use {Encodable, Decodable, Encoder, Decoder, Config};
    use std::io::Cursor;
    use {decode};

    fn roundtrip<A:Encodable, B:Decodable>(v: &A) -> B {
        let mut e = Encoder::new(Vec::new());
        v.encode(&mut e).unwrap();
        let v = e.into_writer();
        println!("Data {:?} {:?}", String::from_utf8_lossy(&v), v);
        let mut d = &mut Decoder::new(Config::default(), Cursor::new(&v[..]));
        decode(d).unwrap()
    }

    #[derive(Debug, PartialEq)]
    enum Multi {
        One,
        Two(u32),
        Three(String, u32),
    }

    probor_enum_encoder_decoder!(Multi {
        #0 One(),
        #1 Two(x #1),
        #5 Three(x #1, y #2),
    });

    #[test]
    fn test() {
        use self::Multi::*;
        assert_eq!(roundtrip::<_, Multi>(&One), One);
        assert_eq!(roundtrip::<_, Multi>(&Two(158)), Two(158));
        assert_eq!(roundtrip::<_, Multi>(&Three("test".to_string(), 22)),
                              Three("test".to_string(), 22));
    }
}
