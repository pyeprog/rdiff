use std::collections::HashMap;

fn pair_2_lines_vec(lines1: &Vec<&str>, lines2: &Vec<&str>) -> Vec<(usize, usize)> {
    let line2_dict: HashMap<&str, usize> = lines2
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, line)| (line, i))
        .collect();

    let mut pairing: HashMap<usize, usize> = HashMap::new();

    let mut pairing_line_of_2: usize = 0;
    for (i, line1) in lines1.iter().enumerate() {
        if let Some(line_no) = line2_dict.get(line1) {
            if pairing_line_of_2 <= *line_no {
                pairing.insert(i, *line_no);
                pairing_line_of_2 = *line_no;
            }
        }
    }

    let mut pairing: Vec<(usize, usize)> = pairing.into_iter().collect();
    pairing.sort();

    return pairing;
}

pub fn create_comparation(lines1: &Vec<&str>, lines2: &Vec<&str>) -> (Vec<String>, Vec<String>) {
    use std::cmp::max;

    let pairing = pair_2_lines_vec(lines1, lines2);

    if pairing.is_empty() {
        return (
            lines1.iter().map(|s| s.to_string()).collect(),
            lines1.iter().map(|s| s.to_string()).collect(),
        );
    }

    let mut res_lines1: Vec<String> = Vec::new();
    let mut res_lines2: Vec<String> = Vec::new();

    let mut last_i: usize = 0;
    let mut last_j: usize = 0;

    for (i, j) in pairing.iter() {
        res_lines1.append(
            &mut (&lines1[last_i..*i])
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        res_lines2.append(
            &mut (&lines2[last_j..*j])
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        if *i - last_i < *j - last_j {
            let n_empty_lines = max(*j as i32 - *i as i32 - last_j as i32 + last_i as i32, 0) as usize;
            res_lines1.extend(vec![String::new(); n_empty_lines]);
        } else if *i - last_i > *j - last_j {
            let n_empty_lines = max(*i as i32 - *j as i32 - last_i as i32 + last_j as i32, 0) as usize;
            res_lines2.extend(vec![String::new(); n_empty_lines]);
        }

        last_i = *i;
        last_j = *j;
    }

    res_lines1.append(
        &mut (&lines1[last_i..])
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );
    res_lines2.append(
        &mut (&lines2[last_j..])
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );

    return (res_lines1, res_lines2);
}
