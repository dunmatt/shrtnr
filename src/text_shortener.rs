//! This module houses all of the various heuristics for shortening regular text.

use crate::SubstringShortener;

#[derive(Debug)]
pub struct TextShortener<'a> {
    original: &'a str,
}

impl<'a> TextShortener<'a> {
    pub fn new(s: &'a str) -> TextShortener<'a> {
        TextShortener { original: s }
    }
}

impl<'a> SubstringShortener for TextShortener<'a> {}
