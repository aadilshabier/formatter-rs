use std::fs::{self, File};
use std::io::{self, prelude::*, BufWriter};

pub fn whitespace<P: AsRef<std::path::Path>>(path: P, target: Option<P>) -> io::Result<()> {
    let buffer = fs::read_to_string(path)?;

    let mut output: Vec<&str> = Vec::with_capacity(buffer.len());

    // Removing whitespace after each line
    buffer
        .lines()
        .rev()
        // (A or B) is the same as ~(~A and ~B)
        .skip_while(|line| !((*line) != "" && !(*line).chars().rev().all(|x| x.is_whitespace())))
        .for_each(|line| output.push(line.trim_end()));

    match target {
        Some(target) => {
            let f = File::create(target)?;
            let mut writer = BufWriter::new(f);
            output
                .iter()
                .rev()
                .for_each(|line| writeln!(writer, "{}", *line).expect("Unable to write to file"));
        }
        None => {
            output
                .into_iter()
                .rev()
                .for_each(|line| println!("{}", line));
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
