#[macro_export]
macro_rules! _probor_index {
    ($n: expr) => { $n }
}

#[macro_export]
macro_rules! _probor_count {
    () => { 0 };
    ($item:ident $($tail:ident)*) => { _probor_count!($($tail)*) + 1 };
}

#[macro_export]
macro_rules! _probor_encode_field {
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt optional) => {
        match $me.$field {
            Some(ref x) => try!(Encodable::encode(x, $encoder)),
            None => try!($encoder.null()),
        }
        debug_assert_eq!($idx, _probor_index!($n));
        $idx += 1;
        ($idx);  // avoids warning, but should be optimized anyway
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt) => {
        try!($crate::Encodable::encode(&$me.$field, $encoder));
        debug_assert_eq!($idx, _probor_index!($n));
        $idx += 1;
        ($idx);  // avoids warning, but should be optimized anyway
    };
}


#[macro_export]
macro_rules! probor_enc_struct {
    ($encoder:expr, $me:expr, { $( $item:ident => ( $($props:tt)* ), )* } )
    => {{
        try!($encoder.array(_probor_count!( $($item)* )));
        let mut idx = 0; // I hope this can be optimized out
        $(
            _probor_encode_field!($encoder, $me, idx, $item $($props)*);
        )*
    }}
}
