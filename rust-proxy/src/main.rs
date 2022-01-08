mod cli;
mod comms;
mod labview;

use comms::{AppListener, MessageFromLV, MessageToLV, CommsError};
use labview::{detect_installations, launch_exe, launch_lv, installs::Bitness};
use log::{debug, error, LevelFilter};
use simple_logger::SimpleLogger;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let config = cli::Configuration::from_env();
    let program_args = cli::program_arguments(std::env::args());
    let cwd = std::env::current_dir().unwrap();

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

    let app_listener = AppListener::new();
    println!("{}", app_listener.port());

    let launch_path = &config.to_launch;

    println!("Launch path: {:?}", launch_path);

    let mut process = launch_process(&config, &app_listener);

    let mut connection = app_listener
        .wait_on_app(config.timeout_secs.unwrap_or(60.0))
        .unwrap();

    process.set_connected().unwrap();

    connection
        .write(MessageToLV::ARGS(&program_args[..]))
        .unwrap();
    connection
        .write(MessageToLV::CCWD(cwd))
        .unwrap();

    loop {
        match connection.read() {
            Ok(MessageFromLV::OUTP(string)) => {
                print!("{}", string);
            }
            Ok(MessageFromLV::EXIT(code)) => {
                std::process::exit(code);
            },
            Err(CommsError::ReadLvMessageError(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //no new messages
            },
            Err(e) => {
                error!("{:?}", e);
                break;
            }
        }
    }

    process.stop_monitor();
}

//todo: Error handling
fn launch_process(config: &cli::Configuration, app_listener: &AppListener) -> labview::process::MonitoredProcess {
    let launch_path = config.to_launch.clone();
    let process = match launch_path
        .extension()
        .map(|ext| ext.to_str().unwrap())
    {
        Some("vi") => {    
            let active_install = find_install(&config.lv_version_string, config.bitness);
            Some(launch_lv(&active_install, launch_path, app_listener.port()).unwrap())
        }
        Some("exe") => {
            Some(launch_exe(launch_path, app_listener.port()).unwrap())
        }
        None => {
            Some(launch_exe(launch_path, app_listener.port()).unwrap())
        }
        Some(extension) => {
            panic!("Unknown extension {:?}", extension); 
        },
    };
    process.unwrap()
}

fn find_install(version_string: &Option<String>, bitness: Bitness) -> labview::installs::LabviewInstall {
    let system_installs = detect_installations().unwrap();
    debug!("{}", system_installs.print_details());
    let active_install = match version_string {
        Some(version) => {
            system_installs
                .get_version(version, bitness)
                .or_else(|| system_installs.get_default())
        },
        None => {
            system_installs.get_default()
        }
    };
    let active_install = active_install.unwrap();
    active_install.clone()
}


#[cfg(test)]
mod test {

}
