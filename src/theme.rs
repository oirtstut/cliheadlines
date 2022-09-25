use termimad::{MadSkin, StyledChar};
use crossterm::style::Color::{Rgb, Yellow};

pub fn default() -> MadSkin{
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb {
        r: 28,
        g: 28,
        b: 28,
    });
    skin.bullet = StyledChar::from_fg_char(Yellow, '+');
    skin.set_headers_fg(Yellow);
    skin.inline_code.set_fg(Rgb {
        r: 255,
        g: 0,
        b: 200,
    });
    skin.italic.set_fg(Rgb {
        r: 215,
        g: 255,
        b: 0,
    });

    skin
}