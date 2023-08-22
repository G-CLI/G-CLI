//! Contains the CLI API Defintion.
//!
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::ffi::OsString;
use std::iter::IntoIterator;
use std::path::PathBuf;
use std::time::Duration;

use crate::labview::installs::Bitness;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Configuration {
    pub to_launch: PathBuf,
    pub verbose: bool,
    pub lv_version_string: Option<String>,
    pub bitness: Bitness,
    pub connect_timeout: Duration,
    /// If kill is Some then the value is a timeout to kill LabVIEW if it isn't already killed.
    pub kill: Option<Duration>,
    /// allows LabVIEW to show dialogs by removing the unattended flag.
    pub allow_dialogs: bool,
    /// Dont launch anything if this is true.
    pub no_launch: bool,
}

impl Configuration {
    /// Load configuration from an arguement array. Intended for testing.
    #[allow(dead_code)]
    pub fn from_arg_array(args: Vec<String>) -> Self {
        let matches = clap_app().try_get_matches_from(args).unwrap();
        Self::args_to_configuration(matches)
    }

    /// Load the configuration from the program arguments.
    /// Will exit the program if the arguments are invalid.
    pub fn from_env() -> Self {
        let matches = clap_app().get_matches();
        Self::args_to_configuration(matches)
    }

    /// Private function to extract the common functionality of moving args to config.
    ///
    /// Panics if args fail validation. Need proper error handling here.
    fn args_to_configuration(args: ArgMatches) -> Self {
        let bitness = if args.contains_id("arch") {
            match args.get_one::<String>("arch").map(|s| s.as_str()) {
                Some("64") => Bitness::X64,
                Some("32") => Bitness::X86,
                Some(other) => panic!("Unknown value for arch: \"{other}\""),
                None => unreachable!(),
            }
        } else if args.get_flag("64bit") {
            Bitness::X64
        } else {
            Bitness::X86
        };

        Self {
            to_launch: PathBuf::from(args.get_one::<String>("app to run").unwrap().to_owned()),
            verbose: args.get_flag("verbose mode"),
            lv_version_string: args
                .get_one::<String>("labview version")
                .map(|str| str.to_owned()),
            bitness,

            // First cant panic due to default values. Second could panic if invalid.
            connect_timeout: Duration::from_millis(*args.get_one::<u64>("timeout (ms)").unwrap()),
            kill: if args.get_flag("kill") {
                //todo handle unwraps. The first should not fail due to default. The second could.
                Some(Duration::from_millis(
                    *args.get_one::<u64>("kill timeout (ms)").unwrap_or(&0),
                ))
            } else {
                None
            },
            allow_dialogs: args.get_flag("allow dialogs"),
            no_launch: args.get_flag("no launch"),
        }
    }
}

/// Returns a fully configured clap app with all the parameters configured.
fn clap_app() -> clap::Command {
    Command::new("G CLI")
        .version(VERSION)
        .about("Connects a LabVIEW app to the command line.")
        .arg(
            Arg::new("verbose mode")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Prints additional details for debugging"),
        )
        .arg(
            Arg::new("labview version")
                .action(ArgAction::Set)
                .long("lv-ver")
                .help("The version of LabVIEW to launch e.g. 2020"),
        )
        .arg(
            Arg::new("arch")
                .long("arch")
                .action(ArgAction::Set)
                .help("Set the bitness of the LabVIEW to run. Either \"64\" or \"32\". Default is 32 and if this is set it will override the --x64 flag."))
        .arg(
            Arg::new("64bit")
                .long("x64")
                .action(ArgAction::SetTrue)
                .help("Set this to launch the 64 bit version of LabVIEW. You should prefer the --arch flag as this will be deprecated in the future."),
        )
        .arg(
            Arg::new("timeout (ms)")
                .long("connect-timeout")
                .alias("timeout")
                .help("The time in ms to wait for the connection from LabVIEW")
                .value_parser(value_parser!(u64))
                .default_value("60000"),
        )
        .arg(
            Arg::new("kill")
            .long("kill")
                .action(ArgAction::SetTrue)
            .help("Forces LabVIEW to exit when the program sends the exit code if set. Use kill-timeout to set a delay before this occurs.")
        )
        .arg(
            Arg::new("kill timeout (ms)")
                .long("kill-timeout")
                .help("The delay before the LabVIEW process is killed if the kill flag is set.")
                .value_parser(value_parser!(u64))
                .default_value("10000")
        )
        .arg(
            Arg::new("allow dialogs")
            .long("allow-dialogs")
            .alias("allowDialogs")
                .action(ArgAction::SetTrue)
            .help("Add this flag to allow LabVIEW to show user dialogs by removing the --unattended flag. Generally not recommended")
        )
        .arg(
            Arg::new("no launch")
            .long("no-launch")
                .action(ArgAction::SetTrue)
            .help("Don't launch your VI or application automatically. You must start it manually.")
        )
        .trailing_var_arg(true)
        .arg(Arg::new("app to run").action(ArgAction::Append).required(true))
}

/// Extract the arguments that are going to be passed to the VI/exe we will run.
pub fn program_arguments<T: IntoIterator<Item = OsString>>(main_args: T) -> Vec<OsString> {
    let args_iter = main_args.into_iter();

    args_iter.skip_while(|s| s != "--").skip(1).collect()
}

#[cfg(test)]
mod tests {

    use std::time::Duration;

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

        assert_eq!(config.to_launch.to_str().unwrap(), "test.vi");
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
    fn allow_dialogs_default() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.allow_dialogs, false);
    }

    #[test]
    fn allow_dialogs_camelcase() {
        let args = vec![
            String::from("g-cli"),
            String::from("--allowDialogs"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.allow_dialogs, true);
    }

    #[test]
    fn allow_dialogs_hyphenated() {
        let args = vec![
            String::from("g-cli"),
            String::from("--allow-dialogs"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.allow_dialogs, true);
    }

    #[test]
    fn no_launch_default() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.no_launch, false);
    }

    #[test]
    fn no_launch_present() {
        let args = vec![
            String::from("g-cli"),
            String::from("--no-launch"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.no_launch, true);
    }

    #[test]
    fn lv_details_32bit() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X86);
    }

    #[test]
    fn lv_details_64bit() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("--x64"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X64);
    }

    #[test]
    fn lv_details_arch_set_32bit() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("--arch"),
            String::from("32"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X86);
    }

    #[test]
    fn lv_details_arch_set_64bit() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("--arch"),
            String::from("64"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X64);
    }

    #[test]
    fn lv_details_arch_overrides_x64_flag() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("--arch"),
            String::from("32"),
            String::from("--x64"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X86);
    }

    #[test]
    fn timeout_not_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(Duration::from_millis(60_000), config.connect_timeout);
    }

    #[test]
    fn timeout_set_old_name() {
        let args = vec![
            String::from("g-cli"),
            String::from("--timeout"),
            String::from("10000"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(Duration::from_millis(10000), config.connect_timeout);
    }

    #[test]
    fn timeout_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("--connect-timeout"),
            String::from("10000"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(Duration::from_millis(10000), config.connect_timeout);
    }

    #[test]
    /// Kill is set with no timeout uses default 10 seconds.
    fn kill_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("--kill"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(Some(Duration::from_secs(10)), config.kill);
    }

    #[test]
    /// Kill is not set.
    fn kill_not_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(None, config.kill);
    }

    #[test]
    /// Kill is set with a timeout in seconds.
    fn kill_set_with_timeout() {
        let args = vec![
            String::from("g-cli"),
            String::from("--kill"),
            String::from("--kill-timeout"),
            String::from("5000"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(Some(Duration::from_millis(5000)), config.kill);
    }

    #[test]
    /// Kill timeout is ignored if kill isnt set.
    fn kill_timeout_but_kill_not_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("--kill-timeout"),
            String::from("5000"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(None, config.kill);
    }

    #[test]
    fn get_program_arguments() {
        let args = vec![
            OsString::from("g-cli"),
            OsString::from("test.vi"),
            OsString::from("--"),
            OsString::from("test1"),
            OsString::from("-t"),
            OsString::from("test2"),
        ];

        let processed = program_arguments(args);

        assert_eq!(
            processed,
            vec![
                OsString::from("test1"),
                OsString::from("-t"),
                OsString::from("test2")
            ]
        );
    }

    #[test]
    fn get_program_arguments_empty() {
        let args = vec![OsString::from("g-cli"), OsString::from("test.vi")];

        let processed = program_arguments(args);

        assert_eq!(processed, Vec::<OsString>::new());
    }
}
