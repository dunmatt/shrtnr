//! Shrtnr is a smart string shortener, suitable for command prompts, title bars, and other cases
//! where a long string might need to fit into a single short line.
//!
//! # Examples
//! ```
//! use shrtnr::shorten;
//!
//! let path = "/home/matt/code/shrtnr/src/lib.rs";
//! let short = shorten(path, 20);
//! assert_eq!(short, "~/cd/srtr/src/lib.rs");
//! let short = shorten(path, 11);
//! assert_eq!(short, "~/c/s/s/lib");
//!
//! let url = "https://github.com/rust-lang/rust/blob/master/src/libstd/net/parser.rs";
//! let sixty = shorten(url, 60);
//! assert_eq!(sixty, "github.com/rust-lang/rust/blob/mastr/src/libstd/nt/parser.rs");
//! ```
//!

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![allow(dead_code)] // TODO: remove me

mod chunker;
mod filename_shortener;
mod text_shortener;
mod uri_shortener;

// TODO: consider adding an `impl` module and moving this (and everything else) there to get
// rid of the `pub(crate)`.
pub(crate) trait SubstringShortener: std::fmt::Debug {
    // TODO: add code here
}

/// Shorten applies a whole slew of heuristics to reduce the length of a string to the given
/// target length.
pub fn shorten<T: AsRef<str>>(s: T, target_length: usize) -> String {
    let s = s.as_ref().trim();
    let chunks = chunker::chunk(s);
    // TODO: split into non-escaped space separated chunks
    // TODO: look for POSIX paths
    // TODO: look for home directories
    // TODO: look for embedded relative directories
    // TODO: look for non-first letters to drop
    // TODO: look for file extensions
    // TODO: look for URIs
    // TODO: look for printable escape sequences
    // TODO: look for scheme
    // TODO: look for TLDs
    // TODO: look for query parameters
    // TODO: look for file extensions
    // TODO: look for non-first letters to drop
    // TODO: look for text (through a language model abstraction)
    // TODO: look for extra whitespace, commas, dashes, repeated punctuation
    // TODO: look for 1337 / emoji replacements
    // TODO: look for articles to drop
    // TODO: look for suffixes to drop
    // TODO: look for abbreviations -> a10n
    // TODO: look for non-first non-last vowels to drop
    // TODO: look for non-first non-last non-syllable-boundary consonants to drop
    // TODO: ConvertToCamelCase
    // TODO: truncate and add ellipsis
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
