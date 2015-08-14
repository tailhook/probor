extern crate cbor;
extern crate byteorder;


mod traits;
mod impls;
mod tuples;
mod sequences;
mod maps;
mod errors;
#[macro_use] mod encode_struct;
#[macro_use] mod decode_struct;
mod enums;
mod macros;
mod util;


pub use traits::Encodable;
pub use traits::Decodable;
pub use traits::Decode;
pub use traits::Input;
pub use traits::Output;
pub use traits::decode;
pub use util::{to_buf, from_slice};

// Convenience reexports
pub use errors::DecodeError;
pub type EncodeError = cbor::EncodeError;

pub use cbor as _cbor; // for use in macros

pub use cbor::Encoder;
pub use cbor::{Decoder, Config};
