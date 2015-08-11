
#[macro_export]
macro_rules! probor_struct {
    ( $( #[$tag:meta] )* struct $name:ident
      { $( $item:ident: $typ:ty => ( $($props:tt)* ), )* }) => {
        $( #[$tag] )*
        struct $name {
            $( $item: $typ, )*
        }

        impl $crate::Encodable for $name {
            fn encode<W: $crate::Output>(&self,
                e: &mut $crate::_cbor::Encoder<W>)
                -> Result<(), $crate::_cbor::EncodeError>
            {
                probor_enc_struct!(e, self, {
                    $( $item => ( $($props)* ), )*
                });
                Ok(())
            }
        }
        impl $crate::Decodable for $name {
            fn decode_opt<R: $crate::Input>(
                d: &mut $crate::_cbor::Decoder<R>)
                -> Result<Option<Self>, $crate::DecodeError>
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
    struct Page {
        url: String => (#0),
        title: String => (#1),
        snippet: Option<String> => (#2 optional),
    });

    probor_struct!(
    #[derive(PartialEq, Eq, Debug)]
    struct SearchResults {
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
