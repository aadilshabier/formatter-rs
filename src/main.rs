extern crate clap;
mod lib;
use clap::{App, Arg, SubCommand};
use std::time::Instant;

fn main() {
    let app_matches = App::new("formatter-rs")
        .version("0.1.2")
        .author("Mohammad Aadil Shabier, aadilshabier1@gmail.com")
        .about("Simple rust file formatter")
        .subcommand(
            SubCommand::with_name("whitespace")
                .about("Removes unwanted whitespace")
                .version("1.2")
                .arg(
                    Arg::with_name("source")
                        .required(true)
                        .long("source")
                        .short("s")
                        .help("Source file to format")
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("target")
                        .long("target")
                        .short("t")
                        .help(
                            "Target location to save the formatted file. Outputs to stdout if not given"
                        )
                        .takes_value(true)
                        .index(2)
                        .conflicts_with("write")
                )
                .arg(
                    Arg::with_name("debug")
                        .long("debug")
                        .short("d")
                        .help("Print debug information"),
                )
                .arg(
                    Arg::with_name("write")
                        .long("write")
                        .short("w")
                        .help("Whether formatted text is written on source")
                )
        )
        .get_matches();

    if let Some(sub_matches) = app_matches.subcommand_matches("whitespace") {
        let instant = Instant::now();

        let source = sub_matches.value_of("file");
        let target = if sub_matches.is_present("write") {
            source
        } else {
            sub_matches.value_of("target")
        };

        lib::whitespace(source.unwrap(), target).unwrap();

        if sub_matches.is_present("debug") {
            println!("Took {} s to execute", instant.elapsed().as_secs_f64());
        }
    }
}
