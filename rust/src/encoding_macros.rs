macro_rules! num {
    ($n:expr) => { $n }
}

macro_rules! count {
    () => { 0 };
    ($item:ident $($tail:ident)*) => { count!($tail) + 1 };
}

macro_rules! encode_field {
    ($encoder:expr, $idx:expr, $field #$n:tt optional) => {
        match self.$field {
            Some(x) => try!(Encodable::encode(x)),
            None => try!(e.null()),
        }
        debug_assert_eq!($idx == num!($n));
        $idx += 1;
    };
    ($encoder:expr, $idx:expr, $field #$n:tt) => {
        try!(Encodable::encode(x));
        debug_assert_eq!($idx == num!($n));
        $idx += 1;
    };
}


#[macro_export]
macro_rules! probor_enc_struct {
    ($encoder:expr, { $( $item:ident => ( $($props:tt)* ), )* } ) => {
        try!($encoder.array(count!( $($item)* )));
        let mut idx = 0; // I hope this can be optimized out
        encode_field!($encoder, idx, $item $($props)*);
    }
}
