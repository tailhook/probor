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

macro_rules! index {
    ($n: expr) => { $n }
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
