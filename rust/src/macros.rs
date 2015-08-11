// --------------------  HELPERS --------------------

macro_rules! index {
    ($n: expr) => { $n }
}

// --------------------  DECODING --------------------

macro_rules! define_var {
    ($name:ident) => {
        let mut $name = None;
    };
}

macro_rules! require_field {
    ($name:ident #$x:tt optional) => {
        $name
    };
    ($name:ident #$x:tt) => {
        try!($name.ok_or(
            $crate::errors::DecodeError::AbsentField(stringify!($name))));
    };
    ($name:ident optional) => {
        $name
    };
    ($name:ident) => {
        try!($name.ok_or(
            $crate::errors::DecodeError::AbsentField(stringify!($name))));
    };
}


macro_rules! parse_fields_num {
    ($decoder:expr, $idx:expr, {}) => {
        try!($decoder.skip()
            .map_err(|e| $crate::errors::DecodeError::SkippingError(e)));
    };
    ($decoder:expr, $idx:expr, {
        $item:ident => ( #$n:tt $($tail:tt)* ),
        $( $nitem:ident => ( $($ntail:tt)* ), )*
    }) => {
        if $idx == index!($n) {
            $item = try!($crate::Decodable::decode_opt($decoder)
                .map_err(|e| $crate::errors::DecodeError::BadFieldValue(
                    stringify!($item), Box::new(e))));
        } else {
            parse_fields_num!($decoder, $idx,
                { $( $nitem => ( $($ntail)* ), )* });
        }
    };
    ($decoder:expr, $idx:expr, {
        $item:ident => ( $($tail:tt)* ),
        $( $nitem:ident => ( $($ntail:tt)* ), )*
    }) => {
        // No field number, just skip it on numeric parsing
        parse_fields_num!($decoder, $idx,
            { $( $nitem => ( $($ntail)* ), )* });
    };
}

#[macro_export]
macro_rules! probor_dec_struct {
    ($decoder:expr, { $( $item:ident => ( $($props:tt)* ), )* } ) => {
        let ( $($item),* ) = {
            $(define_var!($item);)*
            match $decoder.array() {
                Ok(array_len) => {
                    for idx in 0..array_len {
                        parse_fields_num!($decoder, idx,
                            { $( $item => ( $($props)* ), )* });
                    }
                }
                Err(::cbor::DecodeError::UnexpectedType {
                    datatype: ::cbor::types::Type::Null, .. }) => {
                    return Ok(None);
                }
                Err(::cbor::DecodeError::UnexpectedType {
                    datatype: ::cbor::types::Type::Object, .. }) => {
                    unimplemented!(); // Decode as mapping
                }
                Err(e) => {
                    return Err($crate::errors::DecodeError::ExpectationFailed(
                        "array or object expected", e));
                }
            }
            ( $( require_field![ $item $($props)* ] ),* )
        };
    }
}

// --------------------  ENCODING --------------------

macro_rules! count {
    () => { 0 };
    ($item:ident $($tail:ident)*) => { count!($($tail)*) + 1 };
}

macro_rules! encode_field {
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt optional) => {
        match $me.$field {
            Some(ref x) => try!(Encodable::encode(x, $encoder)),
            None => try!($encoder.null()),
        }
        debug_assert_eq!($idx, index!($n));
        $idx += 1;
        ($idx);  // avoids warning, but should be optimized anyway
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt) => {
        try!($crate::Encodable::encode(&$me.$field, $encoder));
        debug_assert_eq!($idx, index!($n));
        $idx += 1;
        ($idx);  // avoids warning, but should be optimized anyway
    };
}


#[macro_export]
macro_rules! probor_enc_struct {
    ($encoder:expr, $me:expr, { $( $item:ident => ( $($props:tt)* ), )* } )
    => {{
        try!($encoder.array(count!( $($item)* )));
        let mut idx = 0; // I hope this can be optimized out
        $(
            encode_field!($encoder, $me, idx, $item $($props)*);
        )*
    }}
}

// --------------------  SIMPLIFIED STRUCT --------------------

#[macro_export]
macro_rules! probor_struct {
    (#[$tag:meta]
     $name:ident { $( $item:ident: $typ:ty => ( $($props:tt)* ), )* }) => {
        #[$tag]
        struct $name {
            $( $item: $typ, )*
        }

        impl $crate::Encodable for $name {
            fn encode<W: $crate::Output>(&self, e: &mut ::cbor::Encoder<W>)
                -> Result<(), ::cbor::EncodeError>
            {
                probor_enc_struct!(e, self, {
                    $( $item => ( $($props)* ), )*
                });
                Ok(())
            }
        }
        impl $crate::Decodable for $name {
            fn decode_opt<R: $crate::Input>(d: &mut ::cbor::Decoder<R>)
                -> Result<Option<Self>, $crate::errors::DecodeError>
            {
                probor_dec_struct!(d, {
                    $( $item => ( $($props)* ), )*
                });
                Ok(Some($name {
                    $( $item: $item, )*
                }))
            }
        }
    }
}


#[cfg(test)]
mod test {
    use cbor::{Encoder};
    use {Encodable};
    use std::io::Cursor;
    use cbor::{Decoder, Config};
    use {decode};

    probor_struct!(
    #[derive(PartialEq, Eq, Debug)]
    Page {
        url: String => (#0),
        title: String => (#1),
        snippet: Option<String> => (#2 optional),
    });

    probor_struct!(
    #[derive(PartialEq, Eq, Debug)]
    SearchResults {
        total_results: u64 => (#0),
        results: Vec<Page> => (#1),
    });


    #[test]
    fn test_encode() {
        let buf = Vec::new();
        let mut enc = Encoder::new(buf);
        SearchResults {
            total_results: 112,
            results: vec![Page {
                url: "http://url1.example.com".to_string(),
                title: "One example".to_string(),
                snippet: None,
            }, Page {
                url: "http://url2.example.com".to_string(),
                title: "Two example".to_string(),
                snippet: Some("Example Two".to_string()),
            }],
        }.encode(&mut enc).unwrap();
        let val = &enc.into_writer()[..];
        let orig = &b"\x82\x18\x70\x82\
            \x83\x77http://url1.example.com\x6bOne example\xf6\
            \x83\x77http://url2.example.com\x6bTwo example\x6bExample Two"[..];
        assert_eq!(val.len(), orig.len());
        assert_eq!(&val, &orig);
    }

    #[test]
    fn test_decode() {
        let orig = &b"\x82\x18\x70\x82\
            \x83\x77http://url1.example.com\x6bOne example\xf6\
            \x83\x77http://url2.example.com\x6bTwo example\x6bExample Two"[..];
        let mut dec = Decoder::new(Config::default(), Cursor::new(orig));
        let val = decode(&mut dec).unwrap();
        assert_eq!(dec.into_reader().position() as usize, orig.len());
        assert_eq!(SearchResults {
            total_results: 112,
            results: vec![Page {
                url: "http://url1.example.com".to_string(),
                title: "One example".to_string(),
                snippet: None,
            }, Page {
                url: "http://url2.example.com".to_string(),
                title: "Two example".to_string(),
                snippet: Some("Example Two".to_string()),
            }],
        }, val);
    }

}
