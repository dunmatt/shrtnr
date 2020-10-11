//! Shrtnr is a smart string shortener, suitable for command prompts, title bars, and other cases
//! where a long string might need to fit into a single short line.
//!
//! # Examples
//! ```
//! use shrtnr::shorten;
//! let url = "https://github.com/rust-lang/rust/blob/master/src/libstd/net/parser.rs";
//! let sixty = shorten(url, 60);
//! assert_eq!(sixty, "github.com/rust-lang/rust/blob/mastr/src/libstd/nt/parser.rs")
//! ```
//!

#![forbid(unsafe_code)]
#![deny(missing_docs)]
