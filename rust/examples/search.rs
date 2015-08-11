#[macro_use] extern crate probor;

use probor::{Encoder, Encodable};
use probor::{Decoder, Config, decode};
use std::io::Cursor;

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


fn main() {
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
    let sr: SearchResults = decode(
        &mut Decoder::new(Config::default(), Cursor::new(enc.into_writer())))
        .unwrap();
    println!("Results {:?}", sr);
}
