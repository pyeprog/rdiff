pub mod file {
    pub fn readlines(path: &str) -> Vec<String> {
        use std::fs;
        let res = fs::read_to_string(path);
        let content: String = res.unwrap_or("".to_owned());
        return content.split("\n").map(|l| l.to_string()).collect();
    }
}


#[cfg(test)]
mod tests {
    use crate::util::file::readlines;
    use std::process::Command;

    fn setup() {
        Command::new("bash")
            .arg("-c")
            .arg("echo yes > /tmp/1.txt && echo no >> /tmp/1.txt")
            .output()
            .expect("failed to create test file");
    }

    fn teardown() {
        Command::new("bash")
            .arg("-c")
            .arg("rm -f /tmp/1.txt")
            .output()
            .expect("fail to rm test file");
    }

    #[test]
    fn test_readlines() {
        setup();

        let lines = readlines("/tmp/1.txt");
        assert_eq!(3, lines.len());
        if let Some(first_line) = lines.get(0) {
            assert_eq!("yes".to_owned(), *first_line);
        }
        if let Some(second_line) = lines.get(1) {
            assert_eq!("no".to_owned(), *second_line);
        }
        if let Some(third_line) = lines.get(2) {
            assert_eq!(String::new(), *third_line);
        }

        let lines = readlines("");
        assert_eq!(1, lines.len());
        if let Some(line) = lines.get(0) {
            assert_eq!(String::new(), *line);
        }

        teardown();
    }
}