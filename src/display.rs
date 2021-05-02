use crate::core::comparing_line::ComparingLine;
use itertools::{Itertools, EitherOrBoth::*};

pub fn print_comparing_lines(lines: &Vec<ComparingLine>) -> () {
    lines.iter().for_each(|l| print_comparing_line(l));
}

fn wrap_line(line: &String, width: usize) -> Vec<String> {
    let mut left_line: &str = line.as_str();
    let mut result: Vec<String> = Vec::new();

    while left_line.len() > width {
        let sub = &left_line[..width];
        result.push(sub.to_string());

        left_line = &left_line[width..];
    }

    let space = " ".repeat(width - left_line.len());
    result.push(left_line.to_string() + &space);

    return result;
}

fn print_comparing_line(line: &ComparingLine) -> () {
    let left_lines = wrap_line(&line.line_left, line.sector_width);
    let right_lines = wrap_line(&line.line_right, line.sector_width);

    let border: String = line.border_char.to_string();

    for z in left_lines.iter().zip_longest(right_lines.iter()) {
        let p = match z {
            Both(l, r) => {
               border.clone() + l + &border + r + &border
            },
            Left(l) => {
                border.clone() + l + &border + " ".repeat(line.sector_width).as_str() + &border
            },
            Right(r) => {
                border.clone() + " ".repeat(line.sector_width).as_str() + &border + r + &border
            }
        };

        println!("{}", p);
    }
}

#[cfg(test)]
mod tests {
    use crate::display::wrap_line;

    #[test]
    fn test_wrap_line() {
        let line = "yesyesyes".to_owned();

        let lines = wrap_line(&line, 3);
        assert_eq!(3, lines.len());
        assert!(lines.iter().all(|l| l.eq("yes")));

        let lines = wrap_line(&line, 10);
        assert_eq!(1, lines.len());
        assert_eq!("yesyesyes ", lines.get(0).unwrap().as_str());

        let lines = wrap_line(&"".to_owned(), 3);
        assert_eq!("   ", lines.get(0).unwrap().as_str());
    }

}