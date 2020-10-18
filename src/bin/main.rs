use terminal::style;

macro_rules! style_test {
    ( $( $name:ident: $sn:expr ),* ) => {
        $(
                let mut s = style!("this line is {}!", $sn.to_string());
                s.$name();
                println!("{}", s);
        )*
    }
}

fn main() {
    style_test! {
        bold: "bold",
        faint: "faint",
        italic: "italic",
        underline: "underline",
        slow_blink: "slow_blink",
        fast_blink: "fast_blink",
        reverse: "reverse",
        conceal: "conceal",
        strikethrough: "strikethrough",
        black: "black",
        red: "red",
        green: "green",
        yellow: "yellow",
        blue: "blue",
        magenta: "magenta",
        cyan: "cyan",
        white: "white",
        gray: "gray",
        bright_red: "bright_red",
        bright_green: "bright_green",
        bright_yellow: "bright_yellow",
        bright_blue: "bright_blue",
        bright_magenta: "bright_magenta",
        bright_cyan: "bright_cyan",
        bright_white: "bright_white",
        bg_black: "bg_black",
        bg_red: "bg_red",
        bg_green: "bg_green",
        bg_yellow: "bg_yellow",
        bg_blue: "bg_blue",
        bg_magenta: "bg_magenta",
        bg_cyan: "bg_cyan",
        bg_white: "bg_white",
        bg_gray: "bg_gray",
        bg_bright_red: "bg_bright_red",
        bg_bright_green: "bg_bright_green",
        bg_bright_yellow: "bg_bright_yellow",
        bg_bright_blue: "bg_bright_blue",
        bg_bright_magenta: "bg_bright_magenta",
        bg_bright_cyan: "bg_bright_cyan",
        bg_bright_white: "bg_bright_white"
    }
}
