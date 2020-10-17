//! This module houses the logic that breaks a string of text into chunks.

use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    filename_shortener::FilenameShortener, text_shortener::TextShortener,
    uri_shortener::UriShortener, SubstringShortener,
};

lazy_static! {
    static ref FILENAME_PREFIX: Regex = Regex::new(r"^(\\\s|\S)*?/(\\\s|\S)*").unwrap();
    static ref URI_PREFIX: Regex = Regex::new(r"^\w+:(//(\w+@)?\w+(:\d+)?)?(\\\s|[^?#[[:space:]]])*(\?(\\\s|[^#[[:space:]]])+)?(#(\\\s|\S)+)?").unwrap();
}

/// Splits a given string into its constituent chunks.
pub(crate) fn chunk<'a>(s: &'a str) -> Vec<Box<dyn SubstringShortener + 'a>> {
    // TODO: replace this with one that scans through the whole thing

    let mut results: Vec<Box<dyn SubstringShortener>> = Vec::new();
    let mut to_chunk = s;

    while to_chunk.len() > 0 {
        if let Some(mtch) = URI_PREFIX.find(to_chunk) {
            let (uri, remainder) = to_chunk.split_at(mtch.end());
            to_chunk = remainder.trim();
            results.push(Box::new(UriShortener::new(uri)));
        } else if let Some(mtch) = FILENAME_PREFIX.find(to_chunk) {
            let (name, remainder) = to_chunk.split_at(mtch.end());
            to_chunk = remainder.trim();
            results.push(Box::new(FilenameShortener::new(name)));
        } else if let Some(word) = to_chunk.split_whitespace().next() {
            let (word, remainder) = to_chunk.split_at(word.len());
            to_chunk = remainder.trim();
            results.push(Box::new(TextShortener::new(word)));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_filenames() {
        assert!(FILENAME_PREFIX.find("~/.cargo/bin").is_some());
        assert!(FILENAME_PREFIX.find("/home/matt/.cargo/bin").is_some());
        assert!(FILENAME_PREFIX.find("../shrtnr/.git/config").is_some());
        assert!(FILENAME_PREFIX.find("./Cargo.toml").is_some());

        assert!(FILENAME_PREFIX.find("Cargo.toml").is_none());
        assert!(FILENAME_PREFIX.find("..").is_none());
        assert!(FILENAME_PREFIX.find("www.google.com").is_none());
        assert!(FILENAME_PREFIX.find("").is_none());
    }

    #[test]
    fn recognizes_uris() {
        assert!(URI_PREFIX.find("https://www.google.com").is_some());
        assert!(URI_PREFIX
            .find("https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#Examples")
            .is_some());
        assert!(URI_PREFIX.find("https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top").is_some());
        assert!(URI_PREFIX.find("ldap://[2001:db8::7]/c=GB?objectClass?one").is_some());
        assert!(URI_PREFIX.find("mailto:John.Doe@example.com").is_some());
        assert!(URI_PREFIX.find("news:comp.infosystems.www.servers.unix").is_some());
        assert!(URI_PREFIX.find("tel:+1-816-555-1212").is_some());
        assert!(URI_PREFIX.find("telnet://192.0.2.16:80/").is_some());
        assert!(URI_PREFIX.find("urn:oasis:names:specification:docbook:dtd:xml:4.1.2").is_some());

        // Invalid because it lacks a scheme
        assert!(FILENAME_PREFIX.find("www.google.com").is_none());
    }

    #[test]
    fn chunks_whole_strings() {
        let chunks = chunk("https://crates.io binaries get installed to ~/.cargo/bin");

        assert!(format!("{:?}", chunks[0]).contains("https://crates.io"));
        assert!(format!("{:?}", chunks[1]).contains("binaries"));
        assert!(format!("{:?}", chunks[chunks.len() - 1]).contains("~/.cargo/bin"));
    }
}
