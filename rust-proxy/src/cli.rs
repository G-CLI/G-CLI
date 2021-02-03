//! Contains the CLI API Defintion.
//!
use clap::{App, AppSettings, Arg, ArgMatches};
use std::iter::IntoIterator;
use std::path::PathBuf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Configuration {
    pub to_launch: PathBuf,
    pub verbose: bool,
}

impl Configuration {
    /// Load configuration from an arguement array. Intended for testing.
    #[allow(dead_code)]
    pub fn from_arg_array(args: Vec<String>) -> Self {
        let matches = clap_app().get_matches_from_safe(args).unwrap();
        Self::args_to_configuration(matches)
    }

    /// Load the configuration from the program arguments.
    /// Will exit the program if the arguments are invalid.
    pub fn from_env() -> Self {
        let matches = clap_app().get_matches();
        Self::args_to_configuration(matches)
    }

    /// Private function to extract the common functionality of moving args to config.
    fn args_to_configuration(args: ArgMatches) -> Self {
        Self {
            to_launch: PathBuf::from(args.value_of("app to run").unwrap().to_owned()),
            verbose: args.is_present("verbose mode"),
        }
    }
}

/// Returns a fully configured clap app with all the parameters configured.
fn clap_app() -> clap::App<'static, 'static> {
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

/// Extract the arguments that are going to be passed to the VI/exe we will run.
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

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.to_launch, "test.vi");
    }

    #[test]
    fn no_verbose_mode() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.verbose, false);
    }

    #[test]
    fn verbose_mode() {
        let args = vec![
            String::from("g-cli"),
            String::from("-v"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.verbose, true);
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
