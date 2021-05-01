use crate::display::Win;
use itertools::{EitherOrBoth::*, Itertools};


pub fn union(win1: &mut Win, win2: &mut Win) -> Win {
    let mut unioned: Vec<String> = Vec::new();

    let has_left_bar = win1.has_left_bar && win2.has_left_bar;
    let has_right_bar = win1.has_right_bar && win2.has_right_bar;
    let has_line_number = win1.has_line_number && win2.has_line_number;

    win1.has_line_number = false;
    win1.has_left_bar = false;
    win1.has_right_bar = false;
    win2.has_left_bar = false;
    win2.has_right_bar = false;
    win2.has_line_number = false;

    for pair in win1.printable().iter().zip_longest(win2.printable().iter()) {
        match pair {
            Both(l, r) => unioned.push(l.clone() + "|" + r.as_str()),
            Left(l) => unioned.push(l.clone() + "|" + " ".repeat(win2.char_width).as_str()),
            Right(r) => unioned.push(" ".to_owned().repeat(win1.char_width) + "|" + &r),
        }
    }

    return Win {
        char_width: win1.char_width + win2.char_width + 1,
        lines: unioned,
        has_left_bar,
        has_right_bar,
        has_line_number,
    };
}
