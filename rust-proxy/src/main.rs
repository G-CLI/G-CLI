mod cli;
mod comms;
mod labview;
mod os_string_support;

use comms::{AppListener, CommsError, MessageFromLV, MessageToLV};
use labview::{detect_installations, installs::Bitness, launch_exe, launch_lv};
use log::{debug, error, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

use os_string_support::join_os_string;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    //wrap the app seperately so destructors are all called
    //before exit.
    let return_code = gcli();
    std::process::exit(return_code);
}

fn gcli() -> i32 {
    let config = cli::Configuration::from_env();
    let program_args = cli::program_arguments(std::env::args_os());
    let cwd = std::env::current_dir().unwrap();

    let log_level = if config.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    let mut logger_config = ConfigBuilder::new();

    logger_config
        .add_filter_allow_str("g_cli")
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_time_format_str("%H:%M:%S%.3f");

    TermLogger::init(
        log_level,
        logger_config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Logger failed to start");

    debug!("G CLI Started - Verbose Mode");
    debug!("Version {}", VERSION);
    debug!(
        "G CLI Arguments: {}",
        std::env::args_os()
            .map(|os_string| os_string.to_string_lossy().into_owned())
            .map(|arg| format!("\"{arg}\"")) //wrap in quotes.
            .collect::<Vec<String>>()
            .join(" ")
    );
    debug!(
        "Arguments passed to LabVIEW: {:?}",
        join_os_string(&program_args, " ")
    );

    //give deprecated warning for no-launch
    if config.no_launch {
        error!("No launch was deprecated for v3.0.0")
    }

    let app_listener = AppListener::new().unwrap();

    let mut process = launch_process(&config, &app_listener);

    let mut connection = app_listener.wait_on_app(config.connect_timeout).unwrap();

    process.set_connected().unwrap();

    connection
        .write(MessageToLV::ARGS(&program_args[..]))
        .unwrap();
    connection.write(MessageToLV::CCWD(cwd)).unwrap();

    let mut exit_code = 0;

    loop {
        match connection.read() {
            Ok(MessageFromLV::OUTP(string)) => {
                print!("{}", string);
            }
            Ok(MessageFromLV::EXIT(code)) => {
                debug!("Recieved exit code {}", code);
                exit_code = code;
                break;
            }
            Err(CommsError::ReadLvMessageError(e))
                if e.kind() == std::io::ErrorKind::WouldBlock =>
            {
                //no new messages
            }
            Err(e) => {
                error!("{:?}", e);
                break;
            }
        }
    }

    process.stop(config.kill);
    debug!("Ending G-CLI with exit code {}", exit_code);
    return exit_code;
}

//todo: Error handling
fn launch_process(
    config: &cli::Configuration,
    app_listener: &AppListener,
) -> labview::process::MonitoredProcess {
    let launch_path = config.to_launch.clone();
    let process = match launch_path.extension().map(|ext| ext.to_str().unwrap()) {
        Some("vi") => {
            let active_install = find_install(&config.lv_version_string, config.bitness);
            Some(
                launch_lv(
                    &active_install,
                    launch_path,
                    app_listener.port(),
                    config.allow_dialogs,
                )
                .unwrap(),
            )
        }
        Some("exe") => Some(launch_exe(launch_path, app_listener.port()).unwrap()),
        None => {
            debug!("No extension in path. Assume it is a .vi");
            //Modify the path to include the .vi. Alias as mutable for this case.
            let mut launch_path = launch_path;
            launch_path.set_extension("vi");

            let active_install = find_install(&config.lv_version_string, config.bitness);
            Some(
                launch_lv(
                    &active_install,
                    launch_path,
                    app_listener.port(),
                    config.allow_dialogs,
                )
                .unwrap(),
            )
        }
        Some(extension) => {
            panic!("Unknown extension {:?}", extension);
        }
    };
    process.unwrap()
}

fn find_install(
    version_string: &Option<String>,
    bitness: Bitness,
) -> labview::installs::LabviewInstall {
    let system_installs = detect_installations().unwrap();
    debug!("{}", system_installs.print_details());
    let active_install = match version_string {
        Some(version) => system_installs
            .get_version(version, bitness)
            .or_else(|| system_installs.get_default()),
        None => system_installs.get_default(),
    };
    let active_install = active_install.unwrap();
    active_install.clone()
}

#[cfg(test)]
mod test {}
