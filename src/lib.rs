//! Utilities for interacting with a terminal.
//!
//! The most common use for this package is to style a string to
//! output to the terminal.
//!
//! ```
//! use terminal::style;
//! fn main() {
//!     let s = style!("{}, {}!", "Hello", "world")
//!                 .black()
//!                 .underline()
//!                 .to_string();
//!     assert_eq!(s, "\x1b[30;4mHello, world!\x1b[0m");
//! }
//! ```
//!
//! You can apply foreground and background colors as well as various
//! attributes like bold, itlic, underline, etc.

pub mod consts;

#[macro_use]
pub mod escape_sequence;

#[macro_use]
pub mod style;
