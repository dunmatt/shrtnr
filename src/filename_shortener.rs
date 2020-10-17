//! This module houses all of the various heuristics for shortening filenames.

use crate::SubstringShortener;

#[derive(Debug)]
pub struct FilenameShortener<'a> {
    original: &'a str,
}

impl<'a> FilenameShortener<'a> {
    pub fn new(s: &'a str) -> FilenameShortener<'a> {
        FilenameShortener { original: s }
    }
}

impl<'a> SubstringShortener for FilenameShortener<'a> {}
