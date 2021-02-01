//! Contains the CLI API Defintion.
//!
use clap::{App, AppSettings, Arg};
use std::iter::IntoIterator;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_app() -> clap::App<'static, 'static> {
    App::new("G CLI")
        .version(VERSION)
        .about("Connects a LabVIEW app to the command line.")
        .arg(
            Arg::with_name("verbose mode")
                .short("v")
                .long("verbose")
                .help("Prints additional details for debugging"),
        )
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("app to run").multiple(true).required(true))
}

pub fn program_arguments<T: IntoIterator<Item = String>>(main_args: T) -> Vec<String> {
    let args_iter = main_args.into_iter();

    args_iter.skip_while(|s| s != "--").skip(1).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_item_to_run() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
            String::from("-t"),
            String::from("test2"),
        ];

        let processed = get_app().get_matches_from_safe(args).unwrap();

        assert_eq!(processed.value_of("app to run"), Some("test.vi"));
    }

    #[test]
    fn get_program_arguments() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
            String::from("-t"),
            String::from("test2"),
        ];

        let processed = program_arguments(args);

        assert_eq!(
            processed,
            vec![
                String::from("test1"),
                String::from("-t"),
                String::from("test2")
            ]
        );
    }

    #[test]
    fn get_program_arguments_empty() {
        let args = vec![String::from("g-cli"), String::from("test.vi")];

        let processed = program_arguments(args);

        assert_eq!(processed, Vec::<String>::new());
    }
}
