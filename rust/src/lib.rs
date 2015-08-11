extern crate cbor;
extern crate byteorder;


mod traits;
mod impls;
mod errors;
#[macro_use] mod encoding_macros;
#[macro_use] mod decoding_macros;
mod macros;


pub use traits::Encodable;
pub use traits::Decodable;
pub use traits::Input;
pub use traits::Output;
pub use traits::decode;

// Convenience reexports
pub use errors::DecodeError;
pub type EncodeError = cbor::EncodeError;

pub use cbor as _cbor; // for use in macros

pub use cbor::Encoder;
pub use cbor::{Decoder, Config};
