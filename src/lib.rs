use std::fs::write;
use std::io;

pub fn whitespace(path: &str, target: Option<&str>) -> io::Result<()> {
    let buffer = std::fs::read_to_string(path)?;
    let size = buffer.len();
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
            write(filename, output)?;
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
