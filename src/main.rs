extern crate clap;
mod lib;
use clap::{App, Arg, SubCommand};

fn main() {
    let app_matches = App::new("formatter-rs")
        .version("0.1.0")
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
                        .help("Removes unwanted whitespace and returns formatted text")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("target")
                        .long("target")
                        .short("t")
                        .help("Target to save the formatted file at")
                        .takes_value(true)
                        .default_value(""),
                ),
        )
        .get_matches();

    //
    if let Some(sub_matches) = app_matches.subcommand_matches("whitespace") {
        lib::whitespace(
            sub_matches.value_of("file").unwrap(),
            sub_matches.value_of("target").unwrap(),
        );
    }
}
