use std::collections::HashMap;

pub fn pair_lines(lines1: &Vec<String>, lines2: &Vec<String>) -> Vec<(usize, usize)> {
    let line_nu_map: HashMap<&String, usize> = lines2
        .iter()
        .enumerate()
        .map(|(nu, line)| (line, nu))
        .collect();

    let mut cur_no_2: usize = 0;
    let mut pair: Vec<(usize, usize)> = Vec::new();

    for (i1, line1) in lines1.iter().enumerate() {
        if let Some(i2) = line_nu_map.get(line1) {
            if cur_no_2 <= *i2 {
                pair.push((i1, *i2));
                cur_no_2 = *i2;
            }
        }
    }

    return pair;
}

#[cfg(test)]
mod tests {
    use crate::core::pairing::pair_lines;

    #[test]
    fn test_pair_lines() {
        let lines1 = vec!["yes".to_owned(), "no".to_owned()];
        let lines2 = vec!["no".to_owned(), "yes".to_owned()];

        let pair = pair_lines(&lines1, &lines2);
        assert_eq!(1, pair.len());
        assert_eq!((0, 1), *pair.get(0).unwrap());
    }

    #[test]
    fn test_pair_empty_lines() {
        let lines1: Vec<String> = Vec::new();
        let lines2: Vec<String> = Vec::new();

        let pair = pair_lines(&lines1, &lines2);
        assert_eq!(0, pair.len());
    }

    #[test]
    fn test_pair_same_lines() {
        let lines1 = vec!["yes".to_owned(), "no".to_owned(), "perhaps".to_owned()];
        let lines2 = lines1.clone();

        let pair = pair_lines(&lines1, &lines2);
        assert_eq!(lines1.len(), pair.len());
    }
}