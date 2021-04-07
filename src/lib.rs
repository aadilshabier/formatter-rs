use std::fs::File;
use std::io::{self, prelude::*};

pub fn whitespace<P>(source: P, target: Option<P>) -> io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let f = File::open(source)?;
    let mut reader = io::BufReader::new(f);

    let mut buf = String::new();
    let mut temp = String::new();
    let mut out = String::new();

    while reader.read_line(&mut buf)? != 0 {
        if !(buf.chars().all(|x| x.is_whitespace())) {
            out.push_str(temp.as_str());
            temp.clear();
            out.push_str(buf.trim_end());
            out.push('\n');
        } else {
            temp.push('\n');
        }
        buf.clear();
    }

    match target {
        Some(target) => {
            let mut file = File::create(target)?;
            file.write_all(out.as_bytes())?;
        }
        None => {
            println!("{}", out);
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
        whitespace("testfiles/testin.txt", Some("tmp/a.txt")).unwrap();

        let expected_output = fs::read_to_string("testfiles/testout.txt").unwrap();
        let test_output = fs::read_to_string("tmp/a.txt").unwrap();
        fs::remove_file("tmp/a.txt").unwrap();

        assert_eq!(test_output, expected_output);
    }
}
