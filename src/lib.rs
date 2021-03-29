use std::fs::File;
use std::io::BufWriter;
use std::io::{prelude::*, stdout};

pub fn whitespace(path: &str, target: &str) {
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

    let output = output.into_iter().rev().collect::<String>();

    // If target not set print to stdout
    if target == "" {
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout.lock());

        writer.write_all(output.as_bytes()).unwrap();
    } else {
        let f = File::create(target).unwrap();
        let mut writer = BufWriter::new(f);

        writer.write_all(output.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        todo!("Rewrite tests");
    }
}
