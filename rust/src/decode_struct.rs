#[macro_export]
macro_rules! _probor_define_var {
    ($name:ident) => {
        let mut $name = None;
    };
}

#[macro_export]
macro_rules! _probor_require_field {
    ($name:ident #$x:tt optional) => {
        $name
    };
    ($name:ident #$x:tt) => {
        try!($name.ok_or(
            $crate::DecodeError::AbsentField(stringify!($name))));
    };
    ($name:ident optional) => {
        $name
    };
    ($name:ident) => {
        try!($name.ok_or(
            $crate::DecodeError::AbsentField(stringify!($name))));
    };
}

#[macro_export]
macro_rules! _probor_parse_fields_num {
    ($decoder:expr, $idx:expr, {}) => {
        try!($decoder.skip()
            .map_err(|e| $crate::DecodeError::SkippingError(e)));
    };
    ($decoder:expr, $idx:expr, {
        $item:ident => ( #$n:tt $($tail:tt)* ),
        $( $nitem:ident => ( $($ntail:tt)* ), )*
    }) => {
        if $idx == _probor_index!($n) {
            $item = try!($crate::Decodable::decode_opt($decoder)
                .map_err(|e| $crate::DecodeError::BadFieldValue(
                    stringify!($item), Box::new(e))));
        } else {
            _probor_parse_fields_num!($decoder, $idx,
                { $( $nitem => ( $($ntail)* ), )* });
        }
    };
    ($decoder:expr, $idx:expr, {
        $item:ident => ( $($tail:tt)* ),
        $( $nitem:ident => ( $($ntail:tt)* ), )*
    }) => {
        // No field number, just skip it on numeric parsing
        _probor_parse_fields_num!($decoder, $idx,
            { $( $nitem => ( $($ntail)* ), )* });
    };
}

#[macro_export]
macro_rules! _probor_dec_struct {
    ($decoder:expr, { $( $item:ident => ( $($props:tt)* ), )* } ) => {
        let ( $($item,)* ) = {
            $(_probor_define_var!($item);)*
            match $decoder.array() {
                Ok(array_len) => {
                    for idx in 0..array_len {
                        (idx+1); // silence warning but expect to be optimized
                        _probor_parse_fields_num!($decoder, idx,
                            { $( $item => ( $($props)* ), )* });
                    }
                }
                Err($crate::_cbor::DecodeError::UnexpectedType {
                    datatype: $crate::_cbor::types::Type::Null, .. }) => {
                    return Ok(None);
                }
                Err($crate::_cbor::DecodeError::UnexpectedType {
                    datatype: $crate::_cbor::types::Type::Object,
                    info: inf @ 0...30
                }) => {
                    let object_num = try!($decoder.kernel().unsigned(inf)
                        .map_err(|e| $crate::DecodeError::WrongType(
                            "array or object expected (e1)", e)));
                    _probor_dec_named!($decoder, object_num,
                        { $( $item => ($($props)* ), )* });
                }
                Err(e) => {
                    return Err($crate::DecodeError::WrongType(
                        "array or object expected (e2)", e));
                }
            }
            ( $( _probor_require_field![ $item $($props)* ], )* )
        };
    }
}

#[macro_export]
macro_rules! _probor_uint_type {
    ($val:expr) => {
        $val == $crate::_cbor::types::Type::UInt8
        || $val == $crate::_cbor::types::Type::UInt16
        || $val == $crate::_cbor::types::Type::UInt32
        || $val == $crate::_cbor::types::Type::UInt64
    }
}

#[macro_export]
macro_rules! _probor_dec_named {
    ($decoder:expr, $nfields:expr,
        { $( $item:ident => ( $($props:tt)* ), )* } )
    => {
        for _ in 0..$nfields {
            match $decoder.text() {
                $(
                    Ok(ref name) if &name[..] == stringify!($item) => {
                        $item = try!($crate::Decodable::decode_opt($decoder)
                            .map_err(|e| $crate::DecodeError::BadFieldValue(
                                stringify!($item), Box::new(e))));
                    }
                )*
                Ok(_) => {
                    try!($decoder.skip().map_err(|e|
                        $crate::DecodeError::SkippingError(e)));
                }
                Err($crate::_cbor::DecodeError::UnexpectedType {
                    datatype: ty, info: inf }) if _probor_uint_type!(ty)
                => {
                    let idx = try!($decoder.kernel().u64(&(ty, inf))
                        .map_err(|e| $crate::DecodeError::WrongType(
                            "array or object expected (e3)", e)));
                    (idx+1); // silence warning but expect to be optimized
                    _probor_parse_fields_num!($decoder, idx,
                        { $( $item => ( $($props)* ), )* });
                }
                Err(e) => {
                    return Err($crate::DecodeError::WrongType(
                        "array or object expected (e4)", e));
                }
            }
        }
    }
}

#[macro_export]
macro_rules! probor_dec_struct {
    ($decoder:expr, { $( $item:ident => ($($props:tt)* ), )* } ) => {
        _probor_dec_struct!($decoder, { $( $item => ($($props)* ), )* });
    };
}
