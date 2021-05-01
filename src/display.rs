pub struct Win {
    pub char_width: usize,
    pub has_line_number: bool ,
    pub has_left_bar: bool ,
    pub has_right_bar: bool ,
    pub lines: Vec<String>,
}

impl Win {
    pub fn printable(&self) -> Vec<String> {
        use std::cmp::max;

        let mut printable: Vec<String> = Vec::new();

        let n_line_number_chars = self.lines.len().to_string().len();

        for (i, line_str_ref) in self.lines.iter().enumerate() {
            let left_bar: String = if self.has_left_bar { "|".to_owned() } else { String::new() };
            let right_bar: String = if self.has_right_bar { "|".to_owned() } else { String::new() };

            let mut prefix: String = if self.has_line_number {
                let line_nu_str = (i + 1).to_string();
                let n_space = max(n_line_number_chars - line_nu_str.len(), 0);
                " ".repeat(n_space) + &line_nu_str + "|"

            } else {
                String::new()
            };

            let n_prefix_chars = prefix.len();

            let mut line_ref: &str = line_str_ref;

            while line_ref.len() > self.char_width {
                let cur_line = left_bar.clone() + &prefix + &line_ref[..self.char_width].to_string() + &right_bar;
                printable.push(cur_line);

                line_ref = &line_ref[self.char_width..];

                prefix = " ".repeat(n_prefix_chars - 1) + "|";
            }

            let space = " ".repeat(max(self.char_width as i32 - line_ref.len() as i32, 0) as usize);

            printable.push(left_bar + &prefix + line_ref + &space + &right_bar);
        }

        return printable;
    }

    pub fn print(&self) -> () {
        for line in self.printable().iter() {
            println!("{}", line);
        }
    }
}
