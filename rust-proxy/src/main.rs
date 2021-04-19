mod cli;
mod comms;
mod labview;

use comms::{AppListener, MessageFromLV, MessageToLV};
use labview::{detect_installations, launch_exe, launch_lv};
use log::{debug, error, LevelFilter};
use simple_logger::SimpleLogger;

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

    let system_installs = detect_installations().unwrap();
    debug!("{}", system_installs.print_details());

    //Todo: need to handle unwrap here with a default version or failure.
    let active_install = system_installs
        .get_version(&config.lv_version_string.unwrap(), config.bitness)
        .unwrap();

    let app_listener = AppListener::new();
    println!("{}", app_listener.port());

    println!("Launch path: {:?}", config.to_launch);

    let process = match config
        .to_launch
        .extension()
        .map(|ext| ext.to_str().unwrap())
    {
        Some("vi") => {
            Some(launch_lv(active_install, config.to_launch, app_listener.port()).unwrap())
        }
        Some("exe") => {
            Some(launch_exe(config.to_launch, app_listener.port()).unwrap())
        }
        None => {
            Some(launch_exe(config.to_launch, app_listener.port()).unwrap())
        }
        Some(extension) => {
            panic!("Unknown extension {:?}", extension); 
        },
    };

    //placeholder for better error handling.
    let mut process = process.unwrap();

    let mut connection = app_listener
        .wait_on_app(config.timeout_secs.unwrap_or(6000.0))
        .unwrap();

    process.set_connected().unwrap();

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

    process.stop_monitor();
}
