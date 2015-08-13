// This shim converts "tt" token into "expr" token (i.e. number)
#[macro_export]
macro_rules! _probor_index {
    ($n: expr) => { $n }
}

#[macro_export]
macro_rules! _probor_max {
    () => { 0 };
    ($n:expr, $($tail:expr,)*) => { ::std::cmp::max(_probor_max!($($tail,)*), $n) };
}

#[macro_export]
macro_rules! _probor_count {
    () => { 0 };
    ($item:ident $($tail:ident)*) => { _probor_count!($($tail)*) + 1 };
}

// TODO(tailhook) shouldn't this be a function?
#[macro_export]
macro_rules! _probor_skip_to {
    ($encoder:expr, $idx:expr, $n:expr) => {
        debug_assert!($idx < $n);
        $idx += 1; // the thing we will subsequently write
        while $idx < $n {
            try!($encoder.null());
            $idx += 1;
        }
    }
}

#[macro_export]
macro_rules! _probor_encode_pos_field {
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt optional) => {
        _probor_skip_to!($encoder, $idx, $n);
        match $me.$field {
            Some(ref x) => try!(Encodable::encode(x, $encoder)),
            None => try!($encoder.null()),
        }
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt) => {
        _probor_skip_to!($encoder, $idx, $n);
        try!($crate::Encodable::encode(&$me.$field, $encoder));
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident optional) => {
        _probor_skip_to!($encoder, $idx, $n);
        match $me.$field {
            Some(ref x) => try!(Encodable::encode(x, $encoder)),
            None => try!($encoder.null()),
        }
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident) => {
        _probor_skip_to!($encoder, $idx, $n);
        try!($crate::Encodable::encode(&$me.$field, $encoder));
    };
}

#[macro_export]
macro_rules! _probor_encode_field {
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt optional) => {
        try!($encoder.u64(_probor_index!($n)));
        match $me.$field {
            Some(ref x) => try!(Encodable::encode(x, $encoder)),
            None => try!($encoder.null()),
        }
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident #$n:tt) => {
        try!($encoder.u64(_probor_index!($n)));
        try!($crate::Encodable::encode(&$me.$field, $encoder));
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident optional) => {
        try!($encoder.text(stringify!($field)));
        match $me.$field {
            Some(ref x) => try!(Encodable::encode(x, $encoder)),
            None => try!($encoder.null()),
        }
    };
    ($encoder:expr, $me:expr, $idx:expr, $field:ident) => {
        try!($encoder.text(stringify!($field)));
        try!($crate::Encodable::encode(&$me.$field, $encoder));
    };
}


#[macro_export]
macro_rules! probor_enc_struct {
    // Every field has a number
    ($encoder:expr, $me:expr, { $( $item:ident => (#$n:tt $($props:tt)* ), )* } )
    => {{
        try!($encoder.array(_probor_max!( $($n,)* ) + 1));
        let mut idx = -1; // I hope this can be optimized out
        $(
            _probor_encode_pos_field!($encoder, $me, idx, $item #$n $($props)*);
        )*
    }};
    ($encoder:expr, $me:expr, { $( $item:ident => ( $($props:tt)* ), )* } )
    => {{
        // TODO(tailhook) only write non-None values
        try!($encoder.object(_probor_count!( $($item)* )));
        $(
            _probor_encode_field!($encoder, $me, idx, $item $($props)*);
        )*
    }};
}
