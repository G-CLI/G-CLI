mod cli;
mod comms;
mod labview;

use comms::{AppConnection, AppListener, MessageFromLV, MessageToLV};
use labview::{detect_installations, launch_exe};
use log::{debug, error, LevelFilter};
use simple_logger::SimpleLogger;
use std::path::PathBuf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let config = cli::Configuration::from_env();
    let program_args = cli::program_arguments(std::env::args());

    let log_level = if config.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    SimpleLogger::new().with_level(log_level).init().unwrap();

    debug!("G CLI Started - Verbose Mode");
    debug!("Version {}", VERSION);
    debug!("G CLI Arguments: TBC");
    debug!("Arguments passed to LabVIEW: {}", program_args.join(" "));

    detect_installations();

    let app_listener = AppListener::new();
    println!("{}", app_listener.port());

    println!("Launch path: {:?}", config.to_launch);

    //let monitor = labview::process::MonitoredProcess::start(PathBuf::from(
    //    "C:\\Program Files (x86)\\National Instruments\\LabVIEW 2011\\LabVIEW.exe",
    //));
    launch_exe(config.to_launch, app_listener.port()).unwrap();

    let mut connection = app_listener.wait_on_app(10.0).unwrap();

    connection
        .write(MessageToLV::ARGS(&program_args[..]))
        .unwrap();
    connection
        .write(MessageToLV::CCWD(std::env::current_dir().unwrap()))
        .unwrap();

    loop {
        match connection.read() {
            Ok(MessageFromLV::OUTP(string)) => {
                print!("{}", string);
            }
            Ok(MessageFromLV::EXIT(code)) => {
                std::process::exit(code);
            }
            Err(e) => {
                error!("{:?}", e);
                break;
            }
        }
    }

    //print!("{}", monitor.check_process_stopped());
    //print!("{}", monitor.check_process_stopped());

    //monitor.stop();
}
