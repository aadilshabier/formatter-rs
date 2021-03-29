use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub fn whitespace(path: &str, target: &str) {
    let mut f = File::open(path).expect(format!("File {} not found", path).as_str());
    let mut buffer = String::new();

    let size = f.read_to_string(&mut buffer).unwrap();
    let mut output: Vec<char> = Vec::with_capacity(size);
    output.push('\n');

    // Checks for whitespace only if check is true
    let mut check = true;

    // Removing whitespace after each line
    for ch in buffer.chars().rev().skip_while(|ch| *ch == '\n') {
        if ch == '\n' {
            check = true;
            output.push('\n');
        } else if check {
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
        print!("{}", output);
    } else {
        let f = File::create(target).unwrap();
        let mut writer = BufWriter::new(f);

        write!(&mut writer, "{}", output).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        let mut expected_output = String::new();
        let mut test_output = String::new();

        File::open("testfiles/output/a.txt")
            .unwrap()
            .read_to_string(&mut expected_output)
            .unwrap();
        whitespace("testfiles/input/a.txt", "tmp/a.txt");

        File::open("tmp/a.txt")
            .unwrap()
            .read_to_string(&mut &mut test_output)
            .unwrap();

        assert_eq!(test_output, expected_output);
    }
}
