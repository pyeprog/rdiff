mod pairing;
mod display;
mod util;

fn main() {
    use std::env;
    use std::fs;
    use display::Win;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: rdiff [file1] [file2]");
        return;
    }

    let content1 = fs::read_to_string(&args[1]).expect("file1");
    let content2 = fs::read_to_string(&args[2]).expect("file2");

    let lines1: Vec<&str> = content1.split('\n').collect();
    let lines2: Vec<&str> = content2.split('\n').collect();

    let (printable1, printable2) = pairing::create_comparation(&lines1, &lines2);

    let mut win1 = Win {
        char_width: 30,
        has_left_bar: true,
        has_right_bar: true,
        has_line_number: true,
        lines: printable1,
    };

    let mut win2 = Win {
        char_width: 30,
        has_left_bar: true,
        has_right_bar: true,
        has_line_number: true,
        lines: printable2,
    };

    let win = util::union(&mut win1, &mut win2);
    win.print();
}
