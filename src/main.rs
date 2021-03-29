extern crate clap;
mod lib;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("formatter-rs")
        .version("1.0")
        .author("Mohammad Aadil Shabier, aadilshabier1@gmail.com")
        .about("Simple rust file formatter")
        .subcommand(
            SubCommand::with_name("whitespace")
                .about("Removes unwanted whitespace")
                .version("1.0")
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .help("Removes unwanted whitespace")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("whitespace") {
        lib::whitespace(matches.value_of("file").unwrap());
    }
}
