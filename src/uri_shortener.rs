//! This module houses all of the various heuristics for shortening URIs.

use crate::SubstringShortener;

#[derive(Debug)]
pub struct UriShortener<'a> {
    original: &'a str,
}

impl<'a> UriShortener<'a> {
    pub fn new(s: &'a str) -> UriShortener<'a> {
        UriShortener { original: s }
    }
}

impl<'a> SubstringShortener for UriShortener<'a> {}
