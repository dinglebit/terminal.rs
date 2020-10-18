//! Escape Sequence for the terminal.
//!
//! There are a variety of escape sequences (see [ANSI escape
//! codes](http://ascii-table.com/ansi-escape-sequences.php)) for the
//! terminal but we currently have only implemented "Set Graphics
//! Mode". More can be added to the enum as needed.
//!
//! # Examples
//!
//! ```
//! # #[macro_use]
//! # use terminal::escape_sequence::EscapeSequence;
//! # use terminal::consts::*;
//! # fn main() {
//!     let s = EscapeSequence::SetGraphicsMode(vec![OP_BOLD, FG_BLACK, BG_WHITE]).to_string();
//!     assert_eq!(s, "\x1b[1;30;47m");
//! # }
//! ```

use std::fmt;

/// An escape sequence for the terminal.
pub enum EscapeSequence {
    /// Change the foreground, background, and attributes of terminal
    /// text using the given values.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # use terminal::escape_sequence::EscapeSequence;
    /// # use terminal::consts::*;
    /// # fn main() {
    ///     let s = EscapeSequence::SetGraphicsMode(vec![OP_BOLD, FG_BLACK, BG_WHITE]).to_string();
    ///     assert_eq!(s, "\x1b[1;30;47m");
    /// # }
    /// ```
    SetGraphicsMode(Vec<u8>),
}

impl fmt::Display for EscapeSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EscapeSequence::SetGraphicsMode(v) => {
                // This seems "faster" per an article I found, but we
                // could also just map/collect/join to a string.
                let mut s = String::with_capacity(100);
                if v.len() > 0 {
                    s.push_str(v[0].to_string().as_str());
                    for i in v[1..].into_iter() {
                        s.push_str(";");
                        s.push_str(i.to_string().as_str());
                    }
                }
                write!(f, "{}", format!("\x1b[{}m", s))
            }
        }
    }
}

/// Create a reset escape sequence for the graphics mode.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # use terminal::reset;
/// # fn main() {
///     let s = reset!().to_string();
///     assert_eq!(s, "\x1b[0m");
/// # }
/// ```
#[macro_export]
macro_rules! reset {
    () => {
        $crate::escape_sequence::EscapeSequence::SetGraphicsMode(vec![$crate::consts::OP_RESET])
    };
}

/// Create a SetGraphicsMode using the given vector.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # use terminal::sgm;
/// # use terminal::consts::*;
/// # fn main() {
///     let s = sgm!(vec![OP_BOLD, FG_BLACK, BG_WHITE]).to_string();
///     assert_eq!(s, "\x1b[1;30;47m");
/// # }
/// ```
#[macro_export]
macro_rules! sgm {
    ($e:expr) => {
        $crate::escape_sequence::EscapeSequence::SetGraphicsMode(($e))
    };
}

#[cfg(test)]
mod tests {
    pub use super::*;
    pub use crate::consts::*;

    #[test]
    fn display() {
        let s = sgm!(vec![OP_BOLD, FG_BLACK, BG_WHITE]);
        assert_eq!("test: \x1b[1;30;47m", format!("test: {}", s));
    }
}
