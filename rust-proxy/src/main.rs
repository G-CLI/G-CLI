mod cli;
mod comms;
mod labview;
mod os_string_support;

use comms::{AppListener, CommsError, MessageFromLV, MessageToLV};
use eyre::{eyre, Report, Result, WrapErr};
use labview::{detect_installations, installs::Bitness, launch_exe, launch_lv};
use log::{debug, error, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

use os_string_support::join_os_string;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Report> {
    //wrap the app seperately so destructors are all called
    //before exit.
    let return_code = gcli()?;
    std::process::exit(return_code);
}

fn gcli() -> Result<i32> {
    let config = cli::Configuration::from_env();
    let program_args = cli::program_arguments(std::env::args_os());
    let cwd = std::env::current_dir().unwrap();

    configure_logger(config.verbose)?;

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

    let app_listener = AppListener::new().wrap_err("Failed to create the network listener")?;
    let mut process =
        launch_process(&config, &app_listener).wrap_err("Failed to launch the process.")?;

    let mut connection = app_listener
        .wait_on_app(config.connect_timeout)
        .wrap_err("No connection established with application.")?;

    process
        .set_connected()
        .wrap_err("Failed to notify the monitoring process of the connection")?;

    connection
        .write(MessageToLV::ARGS(&program_args[..]))
        .wrap_err("Failed to write arguments to LabVIEW application")?;
    connection
        .write(MessageToLV::CCWD(cwd))
        .wrap_err("Failed to write CWD to LabVIEW application")?;

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
    return Ok(exit_code);
}

fn configure_logger(verbose: bool) -> Result<(), Report> {
    let log_level = if verbose {
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
    .wrap_err("Logger failed to start")?;
    Ok(())
}

/// Launch the client process.
///
/// Contains the logic to select different launch methods based on the type of file we are launching.
fn launch_process(
    config: &cli::Configuration,
    app_listener: &AppListener,
) -> Result<labview::process::MonitoredProcess> {
    let launch_path = config.to_launch.clone();
    let extension_as_str = launch_path.extension().map(|ext| {
        //allow panic here as I don't expect we will ever really hit it.
        ext.to_str().expect("Extension isn't valid UTF-8")
    });

    let process = match extension_as_str {
        Some("vi") => {
            let active_install = find_install(&config.lv_version_string, config.bitness)?;

            launch_lv(
                &active_install,
                launch_path,
                app_listener.port(),
                config.allow_dialogs,
            )
            .wrap_err("Failed to Launch LabVIEW")
        }
        Some("exe") => {
            launch_exe(launch_path, app_listener.port()).wrap_err("Failed to Launch Executable")
        }
        None => {
            debug!("No extension in path. Assume it is a .vi");
            //Modify the path to include the .vi. Alias as mutable for this case.
            let mut launch_path = launch_path;
            launch_path.set_extension("vi");

            let active_install = find_install(&config.lv_version_string, config.bitness)?;

            launch_lv(
                &active_install,
                launch_path,
                app_listener.port(),
                config.allow_dialogs,
            )
            .wrap_err("Failed to launch LabVIEW")
        }
        Some(extension) => Err(eyre!("Unknown extension {:?}", extension)),
    };
    process
}

/// Uses the version string to select a valid LabVIEW installation.
fn find_install(
    version_string: &Option<String>,
    bitness: Bitness,
) -> Result<labview::installs::LabviewInstall> {
    let system_installs =
        detect_installations().wrap_err("Failed to run LabVIEW install detection.")?;
    debug!("{}", system_installs.print_details());

    let active_install = match version_string {
        Some(version) => system_installs
            .get_version(version, bitness)
            .or_else(|| system_installs.get_default()),
        None => system_installs.get_default(),
    };

    let active_install = active_install.ok_or_else(|| eyre!("No LabVIEW install found."))?;
    Ok(active_install.clone())
}

#[cfg(test)]
mod test {}
