//! Constants as defined in [ANSI escape
//! code](https://en.wikipedia.org/wiki/ANSI_escape_code).

/// SGR value for op reset.
pub const OP_RESET: u8 = 0;

/// SGR value for op bold.
pub const OP_BOLD: u8 = 1;

/// SGR value for op faint.
pub const OP_FAINT: u8 = 2;

/// SGR value for op italic.
pub const OP_ITALIC: u8 = 3;

/// SGR value for op underline.
pub const OP_UNDERLINE: u8 = 4;

/// SGR value for op slow blink.
pub const OP_SLOW_BLINK: u8 = 5;

/// SGR value for op fast blink.
pub const OP_FAST_BLINK: u8 = 6;

/// SGR value for op reverse.
pub const OP_REVERSE: u8 = 7;

/// SGR value for op conceal.
pub const OP_CONCEAL: u8 = 8;

/// SGR value for op strikethrough.
pub const OP_STRIKETHROUGH: u8 = 9;

/// SRG value for foreground black.
pub const FG_BLACK: u8 = 30;

/// SRG value for foreground red.
pub const FG_RED: u8 = 31;

/// SRG value for foreground green.
pub const FG_GREEN: u8 = 32;

/// SRG value for foreground yellow.
pub const FG_YELLOW: u8 = 33;

/// SRG value for foreground blue.
pub const FG_BLUE: u8 = 34;

/// SRG value for foreground magenta.
pub const FG_MAGENTA: u8 = 35;

/// SRG value for foreground cyan.
pub const FG_CYAN: u8 = 36;

/// SRG value for foreground white.
pub const FG_WHITE: u8 = 37;

/// SRG value for foreground gray.
pub const FG_GRAY: u8 = 90;

/// SRG value for foreground bright red.
pub const FG_BRIGHT_RED: u8 = 91;

/// SRG value for foreground bright green.
pub const FG_BRIGHT_GREEN: u8 = 92;

/// SRG value for foreground bright yellow.
pub const FG_BRIGHT_YELLOW: u8 = 93;

/// SRG value for foreground bright blue.
pub const FG_BRIGHT_BLUE: u8 = 94;

/// SRG value for foreground bright magenta.
pub const FG_BRIGHT_MAGENTA: u8 = 95;

/// SRG value for foreground bright cyan.
pub const FG_BRIGHT_CYAN: u8 = 96;

/// SRG value for foreground bright white.
pub const FG_BRIGHT_WHITE: u8 = 97;

/// SRG value for background black.
pub const BG_BLACK: u8 = 40;

/// SRG value for background red.
pub const BG_RED: u8 = 41;

/// SRG value for background green.
pub const BG_GREEN: u8 = 42;

/// SRG value for background yellow.
pub const BG_YELLOW: u8 = 43;

/// SRG value for background blue.
pub const BG_BLUE: u8 = 44;

/// SRG value for background magenta.
pub const BG_MAGENTA: u8 = 45;

/// SRG value for background cyan.
pub const BG_CYAN: u8 = 46;

/// SRG value for background white.
pub const BG_WHITE: u8 = 47;

/// SRG value for background gray.
pub const BG_GRAY: u8 = 100;

/// SRG value for background bright red.
pub const BG_BRIGHT_RED: u8 = 101;

/// SRG value for background bright green.
pub const BG_BRIGHT_GREEN: u8 = 102;

/// SRG value for background bright yellow.
pub const BG_BRIGHT_YELLOW: u8 = 103;

/// SRG value for background bright blue.
pub const BG_BRIGHT_BLUE: u8 = 104;

/// SRG value for background bright_magenta.
pub const BG_BRIGHT_MAGENTA: u8 = 105;

/// SRG value for background bright_cyan.
pub const BG_BRIGHT_CYAN: u8 = 106;

/// SRG value for background bright white.
pub const BG_BRIGHT_WHITE: u8 = 107;
