//! Apply a style to terminal text using the "Set Graphics Mode"
//! escape sequence.
//!
//! The easiest way to use this is to use the style! macro.
//!
//! ```
//! # #[macro_use]
//! use dinglebit_terminal::style;
//! fn main() {
//!     let s = style!("{}, {}!", "Hello", "world")
//!                 .black()
//!                 .underline()
//!                 .to_string();
//!     assert_eq!(s, "\x1b[30;4mHello, world!\x1b[0m");
//! }
//! ```

use crate::consts::*;
use std::fmt;

/// format! the given values and create a Style that can then be used
/// to style the text.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # use dinglebit_terminal::style;
/// # fn main() {
///     let s = style!("{}, {}!", "Hello", "world")
///                 .black()
///                 .underline()
///                 .to_string();
///     assert_eq!(s, "\x1b[30;4mHello, world!\x1b[0m");
/// # }
/// ```
#[macro_export]
macro_rules! style {
    ($($arg:tt)*) => ($crate::style::Style::new(format!($($arg)*)))
}

/// A Stylized text that can be output to a terminal.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # use dinglebit_terminal::style::Style;
/// # fn main() {
///     let s = Style::new(format!("{}, {}!", "Hello", "world"))
///                 .black()
///                 .underline()
///                 .to_string();
///     assert_eq!(s, "\x1b[30;4mHello, world!\x1b[0m");
/// # }
/// ```
pub struct Style {
    text: String,
    values: Vec<u8>,
}

impl Style {
    /// Create a new style whose text is the given text.
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            values: Vec::new(),
        }
    }

    /// Apply a bold value to the escape sequence.
    pub fn bold(&mut self) -> &mut Self {
        self.values.push(OP_BOLD);
        self
    }

    /// Apply a faint value to the escape sequence.
    pub fn faint(&mut self) -> &mut Self {
        self.values.push(OP_FAINT);
        self
    }

    /// Apply an italic value to the escape sequence.
    pub fn italic(&mut self) -> &mut Self {
        self.values.push(OP_ITALIC);
        self
    }

    /// Apply an underline value to the escape sequence.
    pub fn underline(&mut self) -> &mut Self {
        self.values.push(OP_UNDERLINE);
        self
    }

    /// Apply a slow blink value to the escape sequence.
    pub fn slow_blink(&mut self) -> &mut Self {
        self.values.push(OP_SLOW_BLINK);
        self
    }

    /// Apply a fast blink value to the escape sequence.
    pub fn fast_blink(&mut self) -> &mut Self {
        self.values.push(OP_FAST_BLINK);
        self
    }

    /// Apply a reverse value to the escape sequence.
    pub fn reverse(&mut self) -> &mut Self {
        self.values.push(OP_REVERSE);
        self
    }

    /// Apply a conceal value to the escape sequence.
    pub fn conceal(&mut self) -> &mut Self {
        self.values.push(OP_CONCEAL);
        self
    }

    /// Apply a strikethrough value to the escape sequence.
    pub fn strikethrough(&mut self) -> &mut Self {
        self.values.push(OP_STRIKETHROUGH);
        self
    }

    /// Apply the foreground color black value to the escape sequence.
    pub fn black(&mut self) -> &mut Self {
        self.values.push(FG_BLACK);
        self
    }

    /// Apply the foreground color red value to the escape sequence.
    pub fn red(&mut self) -> &mut Self {
        self.values.push(FG_RED);
        self
    }

    /// Apply the foreground color green value to the escape sequence.
    pub fn green(&mut self) -> &mut Self {
        self.values.push(FG_GREEN);
        self
    }

    /// Apply the foreground color yellow value to the escape
    /// sequence.
    pub fn yellow(&mut self) -> &mut Self {
        self.values.push(FG_YELLOW);
        self
    }

    /// Apply the foreground color blue value to the escape sequence.
    pub fn blue(&mut self) -> &mut Self {
        self.values.push(FG_BLUE);
        self
    }

    /// Apply the foreground color magenta value to the escape
    /// sequence.
    pub fn magenta(&mut self) -> &mut Self {
        self.values.push(FG_MAGENTA);
        self
    }

    /// Apply the foreground color cyan value to the escape sequence.
    pub fn cyan(&mut self) -> &mut Self {
        self.values.push(FG_CYAN);
        self
    }

    /// Apply the foreground color white value to the escape sequence.
    pub fn white(&mut self) -> &mut Self {
        self.values.push(FG_WHITE);
        self
    }

    /// Apply the foreground color gray value to the escape sequence.
    pub fn gray(&mut self) -> &mut Self {
        self.values.push(FG_GRAY);
        self
    }

    /// Apply the foreground color bright red value to the escape
    /// sequence.
    pub fn bright_red(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_RED);
        self
    }

    /// Apply the foreground color bright green value to the escape
    /// sequence.
    pub fn bright_green(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_GREEN);
        self
    }

    /// Apply the foreground color bright yellow value to the escape
    /// sequence.
    pub fn bright_yellow(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_YELLOW);
        self
    }

    /// Apply the foreground color bright blue value to the escape
    /// sequence.
    pub fn bright_blue(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_BLUE);
        self
    }

    /// Apply the foreground color bright magenta value to the escape
    /// sequence.
    pub fn bright_magenta(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_MAGENTA);
        self
    }

    /// Apply the foreground color bright cyan value to the escape
    /// sequence.
    pub fn bright_cyan(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_CYAN);
        self
    }

    /// Apply the foreground color bright white value to the escape
    /// sequence.
    pub fn bright_white(&mut self) -> &mut Self {
        self.values.push(FG_BRIGHT_WHITE);
        self
    }

    /// Apply the background color black value to the escape sequence.
    pub fn bg_black(&mut self) -> &mut Self {
        self.values.push(BG_BLACK);
        self
    }

    /// Apply the background color red value to the escape sequence.
    pub fn bg_red(&mut self) -> &mut Self {
        self.values.push(BG_RED);
        self
    }

    /// Apply the background color green value to the escape sequence.
    pub fn bg_green(&mut self) -> &mut Self {
        self.values.push(BG_GREEN);
        self
    }

    /// Apply the background color yellow value to the escape
    /// sequence.
    pub fn bg_yellow(&mut self) -> &mut Self {
        self.values.push(BG_YELLOW);
        self
    }

    /// Apply the background color blue value to the escape sequence.
    pub fn bg_blue(&mut self) -> &mut Self {
        self.values.push(BG_BLUE);
        self
    }

    /// Apply the background color magenta value to the escape
    /// sequence.
    pub fn bg_magenta(&mut self) -> &mut Self {
        self.values.push(BG_MAGENTA);
        self
    }

    /// Apply the background color cyan value to the escape sequence.
    pub fn bg_cyan(&mut self) -> &mut Self {
        self.values.push(BG_CYAN);
        self
    }

    /// Apply the background color white value to the escape sequence.
    pub fn bg_white(&mut self) -> &mut Self {
        self.values.push(BG_WHITE);
        self
    }

    /// Apply the background color gray value to the escape sequence.
    pub fn bg_gray(&mut self) -> &mut Self {
        self.values.push(BG_GRAY);
        self
    }

    /// Apply the background color bright red value to the escape
    /// sequence.
    pub fn bg_bright_red(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_RED);
        self
    }

    /// Apply the background color bright green value to the escape
    /// sequence.
    pub fn bg_bright_green(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_GREEN);
        self
    }

    /// Apply the background color bright yellow value to the escape
    /// sequence.
    pub fn bg_bright_yellow(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_YELLOW);
        self
    }

    /// Apply the background color bright blue value to the escape
    /// sequence.
    pub fn bg_bright_blue(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_BLUE);
        self
    }

    /// Apply the background color bright magenta value to the escape
    /// sequence.
    pub fn bg_bright_magenta(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_MAGENTA);
        self
    }

    /// Apply the background color bright cyan value to the escape
    /// sequence.
    pub fn bg_bright_cyan(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_CYAN);
        self
    }

    /// Apply the background color bright white value to the escape
    /// sequence.
    pub fn bg_bright_white(&mut self) -> &mut Self {
        self.values.push(BG_BRIGHT_WHITE);
        self
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", sgm!(self.values.clone()), self.text, reset!())
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;
    pub use crate::consts::*;

    macro_rules! style_test {
        ( $( $name:ident: $exp:expr ),* ) => {
            $(
                #[test]
                fn $name() {
                    let mut s = Style::new("test".to_string());
                    s.$name();
                    assert_eq!($exp, s.values);
                }
            )*
        }
    }

    style_test! {
        bold: vec![OP_BOLD],
        faint: vec![OP_FAINT],
        italic: vec![OP_ITALIC],
        underline: vec![OP_UNDERLINE],
        slow_blink: vec![OP_SLOW_BLINK],
        fast_blink: vec![OP_FAST_BLINK],
        reverse: vec![OP_REVERSE],
        conceal: vec![OP_CONCEAL],
        strikethrough: vec![OP_STRIKETHROUGH],
        black: vec![FG_BLACK],
        red: vec![FG_RED],
        green: vec![FG_GREEN],
        yellow: vec![FG_YELLOW],
        blue: vec![FG_BLUE],
        magenta: vec![FG_MAGENTA],
        cyan: vec![FG_CYAN],
        white: vec![FG_WHITE],
        gray: vec![FG_GRAY],
        bright_red: vec![FG_BRIGHT_RED],
        bright_green: vec![FG_BRIGHT_GREEN],
        bright_yellow: vec![FG_BRIGHT_YELLOW],
        bright_blue: vec![FG_BRIGHT_BLUE],
        bright_magenta: vec![FG_BRIGHT_MAGENTA],
        bright_cyan: vec![FG_BRIGHT_CYAN],
        bright_white: vec![FG_BRIGHT_WHITE],
        bg_black: vec![BG_BLACK],
        bg_red: vec![BG_RED],
        bg_green: vec![BG_GREEN],
        bg_yellow: vec![BG_YELLOW],
        bg_blue: vec![BG_BLUE],
        bg_magenta: vec![BG_MAGENTA],
        bg_cyan: vec![BG_CYAN],
        bg_white: vec![BG_WHITE],
        bg_gray: vec![BG_GRAY],
        bg_bright_red: vec![BG_BRIGHT_RED],
        bg_bright_green: vec![BG_BRIGHT_GREEN],
        bg_bright_yellow: vec![BG_BRIGHT_YELLOW],
        bg_bright_blue: vec![BG_BRIGHT_BLUE],
        bg_bright_magenta: vec![BG_BRIGHT_MAGENTA],
        bg_bright_cyan: vec![BG_BRIGHT_CYAN],
        bg_bright_white: vec![BG_BRIGHT_WHITE]
    }

    #[test]
    fn multi_style() {
        let mut s = style!("test");
        s.bold().underline().red().bg_green();
        assert_eq!(s.values, vec![OP_BOLD, OP_UNDERLINE, FG_RED, BG_GREEN]);
        assert_eq!(
            format!(
                "\x1b[{};{};{};{}mtest\x1b[0m",
                OP_BOLD, OP_UNDERLINE, FG_RED, BG_GREEN
            ),
            s.to_string()
        );
    }
}
