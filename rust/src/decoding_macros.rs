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
macro_rules! probor_dec_struct {
    ($decoder:expr, { $( $item:ident => ( $($props:tt)* ), )* } ) => {
        let ( $($item),* ) = {
            $(_probor_define_var!($item);)*
            match $decoder.array() {
                Ok(array_len) => {
                    for idx in 0..array_len {
                        _probor_parse_fields_num!($decoder, idx,
                            { $( $item => ( $($props)* ), )* });
                    }
                }
                Err($crate::_cbor::DecodeError::UnexpectedType {
                    datatype: $crate::_cbor::types::Type::Null, .. }) => {
                    return Ok(None);
                }
                Err($crate::_cbor::DecodeError::UnexpectedType {
                    datatype: $crate::_cbor::types::Type::Object, .. }) => {
                    unimplemented!(); // Decode as mapping
                }
                Err(e) => {
                    return Err($crate::DecodeError::ExpectationFailed(
                        "array or object expected", e));
                }
            }
            ( $( _probor_require_field![ $item $($props)* ] ),* )
        };
    }
}
