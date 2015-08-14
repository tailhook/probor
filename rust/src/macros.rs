#[macro_export]
macro_rules! probor_struct_encoder {
    ($name:ident { $( $item:ident => ( $($props:tt)* ), )* }
    ) => {
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
    }
}

#[macro_export]
macro_rules! probor_struct_decoder {
    ($name:ident { $( $item:ident => ( $($props:tt)* ), )* }
    ) => {
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
#[macro_export]
macro_rules! probor_struct_encoder_decoder {
    ($name:ident { $( $item:ident => ( $($props:tt)* ), )* }
    ) => {
        probor_struct_encoder!($name { $( $item => ( $($props)* ), )* } );
        probor_struct_decoder!($name { $( $item => ( $($props)* ), )* } );
    }
}

/// Declares structure
#[macro_export]
macro_rules! probor_struct {
    // Public struct with public fields
    ( $( #[$tag:meta] )* pub struct $name:ident
      { $(pub $item:ident: $typ:ty => ( $($props:tt)* ), )* }
    ) => {
        $( #[$tag] )*
        pub struct $name {
            $(pub $item: $typ, )*
        }

        probor_struct_encoder_decoder!($name
            { $( $item => ( $($props)* ), )* });
    };
    // Public struct with private fields
    ( $( #[$tag:meta] )* pub struct $name:ident
      { $( $item:ident: $typ:ty => ( $($props:tt)* ), )* }
    ) => {
        $( #[$tag] )*
        pub struct $name {
            $( $item: $typ, )*
        }

        probor_struct_encoder_decoder!($name
            { $( $item => ( $($props)* ), )* });
    };
    // Private struct with public fields
    ( $( #[$tag:meta] )* struct $name:ident
      { $(pub $item:ident: $typ:ty => ( $($props:tt)* ), )* }
    ) => {
        $( #[$tag] )*
        struct $name {
            $(pub $item: $typ, )*
        }

        probor_struct_encoder_decoder!($name
            { $( $item => ( $($props)* ), )* });
    };
    // Private struct with private fields
    ( $( #[$tag:meta] )* struct $name:ident
      { $( $item:ident: $typ:ty => ( $($props:tt)* ), )* }
    ) => {
        $( #[$tag] )*
        struct $name {
            $( $item: $typ, )*
        }

        probor_struct_encoder_decoder!($name
            { $( $item => ( $($props)* ), )* });
    };
}

#[cfg(test)]
mod test_search_results {
    use {Encodable, Decodable, Encoder, Decoder, Config};
    use std::io::Cursor;
    use {decode, to_buf, from_slice};

    fn roundtrip<A:Encodable, B:Decodable>(v: A) -> B {
        let v = to_buf(&v);
        println!("Data {:?} {:?}", String::from_utf8_lossy(&v), v);
        from_slice(&v).unwrap()
    }

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

    probor_struct!(
    pub struct VersionInfo {
        pub version: u8 => (),
    });


    #[test]
    fn one_field() {
        let v = VersionInfo { version: 1 };
        let mut e = Encoder::new(Vec::new());
        v.encode(&mut e).unwrap();
        let vec = e.into_writer();
        assert_eq!(&vec[..], b"\xa1\x67version\x01");
        let mut d = &mut Decoder::new(Config::default(), Cursor::new(vec));
        let ver: VersionInfo = decode(d).unwrap();
        assert_eq!(ver.version, 1);
    }

    probor_struct!(
    #[derive(Debug, PartialEq)]
    struct Three {
        one: Option<u8> => (#0 optional),
        two: Option<u8> => (#1 optional),
        three: Option<u8> => (#2 optional),
    });

    probor_struct!(
    struct TwoThree {
        two: Option<u8> => (#1 optional),
        three: Option<u8> => (#2 optional),
    });

    probor_struct!(
    struct OneThree {
        one: Option<u8> => (#0 optional),
        three: Option<u8> => (#2 optional),
    });

    probor_struct!(
    struct OneTwo {
        one: Option<u8> => (#0 optional),
        two: Option<u8> => (#1 optional),
    });

    probor_struct!(
    struct Named {
        one: u8 => (),
        two: u8 => (),
        three: u8 => (),
    });

    #[test]
    fn big_to_small() {
        let t = Three { one: Some(10), two: Some(20), three: Some(30) };
        let mut e = Encoder::new(Vec::new());
        t.encode(&mut e).unwrap();
        let vec = e.into_writer();
        assert_eq!(&vec[..], b"\x83\x0a\x14\x18\x1e");
        let v: Three = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.one, Some(10));
        assert_eq!(v.two, Some(20));
        assert_eq!(v.three, Some(30));
        let v: TwoThree = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.two, Some(20));
        assert_eq!(v.three, Some(30));
        let v: OneTwo = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.one, Some(10));
        assert_eq!(v.two, Some(20));
        let v: OneThree = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.one, Some(10));
        assert_eq!(v.three, Some(30));
    }


    #[test]
    fn small_to_big() {
        assert_eq!(roundtrip::<_, Three>(OneTwo { one: Some(11), two: Some(22) }),
            Three { one: Some(11), two: Some(22), three: None });
        assert_eq!(roundtrip::<_, Three>(TwoThree { two: Some(32), three: Some(33) }),
            Three { one: None, two: Some(32), three: Some(33) });
        assert_eq!(roundtrip::<_, Three>(OneThree { one: Some(41), three: Some(43) }),
            Three { one: Some(41), two: None, three: Some(43) });
    }

    #[test]
    fn named_to_small() {
        let t = Named { one: 11, two: 21, three: 31 };
        let mut e = Encoder::new(Vec::new());
        t.encode(&mut e).unwrap();
        let vec = e.into_writer();
        assert_eq!(&vec[..], b"\xa3\x63one\x0b\x63two\x15\x65three\x18\x1f");
        let v: Three = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.one, Some(11));
        assert_eq!(v.two, Some(21));
        assert_eq!(v.three, Some(31));
        let v: TwoThree = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.two, Some(21));
        assert_eq!(v.three, Some(31));
        let v: OneTwo = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.one, Some(11));
        assert_eq!(v.two, Some(21));
        let v: OneThree = decode(&mut Decoder::new(
            Config::default(), Cursor::new(&vec[..]))).unwrap();
        assert_eq!(v.one, Some(11));
        assert_eq!(v.three, Some(31));
    }

}
