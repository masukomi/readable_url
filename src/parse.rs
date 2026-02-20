use std::default::Default;
use std::io;

use html5ever::parse_document;
use markup5ever_rcdom::RcDom;
use tendril::TendrilSink;

// Read from stdin and parse as HTML/XML
pub fn parse_stdin() -> RcDom {
    let stdin = io::stdin();
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap()
}

/// Read the provided string and parse as HTML/XML
pub fn parse_string(text: &str) -> RcDom {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap()
}
