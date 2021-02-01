mod cli;
mod comms;
mod labview;

use labview::detect_installations;
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;
use std::path::PathBuf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = cli::get_app().get_matches();
    let program_args = cli::program_arguments(std::env::args());

    let log_level = if args.is_present("verbose mode") {
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

    let monitor = labview::process::MonitoredProcess::start(PathBuf::from(
        "C:\\Program Files (x86)\\National Instruments\\LabVIEW 2011\\LabVIEW.exe",
    ));

    std::thread::sleep(std::time::Duration::from_millis(10000));

    print!("{}", monitor.check_process_stopped());
    print!("{}", monitor.check_process_stopped());

    monitor.stop();
}
