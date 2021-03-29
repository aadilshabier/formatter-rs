use std::fs::File;
use std::io::prelude::*;

pub fn whitespace(path: &str) -> String {
    let mut f = File::open(path).expect(format!("File {} not found", path).as_str());
    let mut buffer = String::new();

    let size = f.read_to_string(&mut buffer).unwrap();
    let mut output: Vec<char> = Vec::with_capacity(size);

    // Checks for whitespace only if check is true
    let mut check = false;

    // If non whitespace character is seen for the first time,
    // set output to
    let mut first = true;

    // Removing whitespace after each line
    for ch in buffer.chars().rev() {
        if ch == '\n' {
            check = true;
            output.push('\n');
        } else if check {
            if first {
                output.clear();
                output.push('\n');
                first = false;
            }
            if ch != ' ' {
                check = false;
                output.push(ch);
            }
        } else {
            output.push(ch);
        }
    }
    output.into_iter().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        let mut f = File::open("testfiles/output/a.txt").unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        assert_eq!(whitespace("testfiles/input/a.txt"), buffer);
    }
}
