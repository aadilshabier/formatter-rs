use std::fs::File;
use std::io::{self, prelude::*, BufWriter};

pub fn whitespace(path: &str, target: Option<&str>) -> io::Result<()> {
    let mut f = File::open(path).expect(format!("File {} not found", path).as_str());
    let mut buffer = String::new();

    let size = f.read_to_string(&mut buffer)?;
    let mut output: Vec<char> = vec!['\n'];
    output.reserve(size - 1);

    // Checks for whitespace only if check is true
    let mut check = true;

    // Removing whitespace after each line
    for ch in buffer.chars().rev().skip_while(|ch| (*ch).is_whitespace()) {
        if ch == '\n' {
            check = true;
            output.push('\n');
        } else if check {
            if !ch.is_whitespace() {
                check = false;
                output.push(ch);
            }
        } else {
            output.push(ch);
        }
    }

    let output = output.into_iter().rev().collect::<String>();

    // If target not set print to stdout

    match target {
        Some(filename) => {
            let f = File::create(filename).unwrap();
            let mut writer = BufWriter::new(f);

            write!(writer, "{}", output).unwrap();
        }
        None => {
            print!("{}", output);
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_whitespace() {
        whitespace("testfiles/input/a.txt", Some("tmp/a.txt")).unwrap();

        let expected_output = fs::read_to_string("testfiles/output/a.txt").unwrap();
        let test_output = fs::read_to_string("tmp/a.txt").unwrap();
        fs::remove_file("tmp/a.txt").unwrap();

        assert_eq!(test_output, expected_output);
    }
}
