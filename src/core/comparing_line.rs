use crate::core::pairing::pair_lines;
use itertools::{Itertools, EitherOrBoth::*};

pub struct ComparingLine {
    pub line_left: String,
    pub line_right: String,
    pub sector_width: usize,
    pub border_char: char,
}

impl ComparingLine {
    fn _longest_zip(lines1: &Vec<String>, lines2: &Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut result: (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());

        for z in lines1.iter().zip_longest(lines2.iter()) {
            match z {
                Both(l1, l2) => {
                    result.0.push(l1.to_string());
                    result.1.push(l2.to_string());
                },
                Left(l1) => {
                    result.0.push(l1.to_string());
                    result.1.push(String::new());
                },
                Right(l2) => {
                    result.0.push(String::new());
                    result.1.push(l2.to_string());
                }
            }
        }

        return result;
    }

    fn _form(lines1: Vec<String>,
             lines2: Vec<String>,
             sector_width: Option<usize>,
             border: Option<char>) -> Vec<ComparingLine> {
        let width = sector_width.unwrap_or(50);
        let border: char = border.unwrap_or('|');

        lines1.iter()
            .zip(lines2.iter())
            .map(|(l1, l2)| ComparingLine {
                line_left: l1.to_string(),
                line_right: l2.to_string(),
                sector_width: width,
                border_char: border,
            })
            .collect()
    }

    pub fn from(lines1: Vec<String>,
                lines2: Vec<String>,
                sector_width: Option<usize>,
                border: Option<char>) -> Vec<ComparingLine> {
        let pairing: Vec<(usize, usize)> = pair_lines(&lines1, &lines2);

        if pairing.is_empty() {
            let (lines1, lines2) = ComparingLine::_longest_zip(&lines1, &lines2);
            return ComparingLine::_form(lines1, lines2, sector_width, border);
        }

        let mut result: Vec<ComparingLine> = Vec::new();

        let mut last_i: usize = 0;
        let mut last_j: usize = 0;

        for (i, j) in pairing.into_iter() {
            let (sector1, sector2) = ComparingLine::_longest_zip(
                &lines1[last_i..i].to_vec(), &lines2[last_j..j].to_vec());
            result.extend(ComparingLine::_form(sector1, sector2, sector_width, border));

            last_i = i;
            last_j = j;
        }

        let (sector1, sector2) = ComparingLine::_longest_zip(
            &lines1[last_i..].to_vec(), &lines2[last_j..].to_vec());
        result.extend(ComparingLine::_form(sector1, sector2, sector_width, border));

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::comparing_line::ComparingLine;

    #[test]
    fn test_longest_zip() {
        let lines1 = vec!["yes".to_owned(), "no".to_owned(), "perhaps".to_owned()];
        let lines2: Vec<String> = Vec::new();

        let (left, right) = ComparingLine::_longest_zip(&lines1, &lines2);
        assert_eq!(3, left.len());
        assert_eq!(3, right.len());

        assert!(lines2.iter().all(|l| l.is_empty()));
        assert!(!lines1.iter().any(|l| l.is_empty()))
    }

    #[test]
    fn test_form() {
        let lines1 = vec!["yes".to_owned(), "no".to_owned(), "perhaps".to_owned()];
        let lines2: Vec<String> = lines1.clone();

        let comparing_lines = ComparingLine::_form(lines1, lines2, Option::None, Option::None);
        assert_eq!(3, comparing_lines.len());
        assert!(comparing_lines.iter().all(|cl| cl.line_left.eq(&cl.line_right)));
    }

    #[test]
    fn test_from() {
        let lines1 = vec!["yes".to_owned(), "no".to_owned()];
        let lines2 = vec!["no".to_owned(), "yes".to_owned()];

        let comparing_lines = ComparingLine::from(lines1, lines2, Option::None, Option::None);
        assert_eq!(3, comparing_lines.len());

        assert!("".eq(&comparing_lines.get(0).unwrap().line_left));
        assert!("no".eq(&comparing_lines.get(0).unwrap().line_right));

        assert!("yes".eq(&comparing_lines.get(1).unwrap().line_left));
        assert!("yes".eq(&comparing_lines.get(1).unwrap().line_right));

        assert!("no".eq(&comparing_lines.get(2).unwrap().line_left));
        assert!("".eq(&comparing_lines.get(2).unwrap().line_right));
    }
}
