use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn whitespace(path: &str) -> String {
    let mut output = String::new();
    let f = File::open(path).expect(format!("File {} not found", path).as_str());

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        let mut f1 = File::open("testfiles/input/a.txt").unwrap();
        let mut f2 = File::open("testfiles/output/a.txt").unwrap();

        let mut buffer1 = String::new();
        let mut buffer2 = String::new();

        f1.read_to_string(&mut buffer1).unwrap();
        f2.read_to_string(&mut buffer2).unwrap();

        assert_eq!(buffer1, buffer2);
    }
}
