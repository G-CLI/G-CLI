mod action_loop;
mod cli;
mod comms;
mod comms_loop;
mod labview;
mod os_string_support;
mod signal_loop;
mod stdin_loop;

use comms::{AppListener, MessageToLV};
use eyre::{Report, Result, WrapErr, eyre};
use labview::{detect_installations, installs::Bitness, launch_exe, launch_lv};
use log::{LevelFilter, debug, error};
use os_string_support::join_os_string;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use time::macros::format_description;

use crate::action_loop::{ActionLoop, ExitAction};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    // Second, dedicated connection for stdin (and future signals like Ctrl+C, see #188).
    // Kept separate from the main comms channel so LabVIEW code consuming stdin doesn't
    // have to filter out unrelated message types (OUTP/SERR/EXIT/...).
    // Diagnostic escape hatch: --no-signal skips this entirely so the main
    // channel can be tested in isolation.
    let signal_listener = if config.no_signal_channel {
        debug!("Signal channel disabled (--no-signal)");
        None
    } else {
        Some(
            AppListener::new()
                .wrap_err("Failed to create the signal channel network listener")?,
        )
    };

    debug!(
        "Waiting for connections - main channel port {}, signal channel port {}",
        app_listener.port(),
        signal_listener
            .as_ref()
            .map(|l| l.port().to_string())
            .unwrap_or_else(|| "disabled".to_string())
    );

    let process = launch_process(&config, &app_listener, signal_listener.as_ref())
        .wrap_err("Failed to launch the process.")?;

    // Wait for both connections before writing anything, so we never write to
    // a half-established pair of channels.
    let mut connection = app_listener
        .wait_on_app(config.connect_timeout)
        .wrap_err("No connection established with application.")?;
    debug!("Main channel connected (port {})", app_listener.port());
    process
        .set_main_connected()
        .wrap_err("Failed to notify the monitoring process of the main channel connection")?;

    let signal_connection = match &signal_listener {
        Some(signal_listener) => {
            let signal_connection = signal_listener
                .wait_on_app(config.connect_timeout)
                .wrap_err("No connection established on the signal channel with application.")?;
            debug!(
                "Signal channel connected (port {})",
                signal_listener.port()
            );
            process.set_signal_connected().wrap_err(
                "Failed to notify the monitoring process of the signal channel connection",
            )?;
            Some(signal_connection)
        }
        None => None,
    };

    connection
        .write(MessageToLV::Args(&program_args[..]))
        .wrap_err("Failed to write arguments to LabVIEW application")?;
    connection
        .write(MessageToLV::Ccwd(cwd))
        .wrap_err("Failed to write CWD to LabVIEW application")?;
    debug!("Sent ARGS and CCWD on main channel");

    // At this point we spawn multiple tasks as processes:
    // 1. Action Loop - Receives messages from inputs and takes appropriate actions.
    //                  Also writes a stop signal for other threads.
    // 2. Comms Loop - Receive incoming comms from LabVIEW.
    // 3. Stdin Loop - Read from stdin and send commands to LabVIEW over the signal channel.
    // 4. CtrlC Handler

    let action_loop = ActionLoop::new();

    comms_loop::start(
        connection,
        action_loop.get_channel(),
        action_loop.get_stop_signal(),
    );

    // signal_loop needs its own handle to the signal connection (independent
    // of stdin_loop's) so it can send SIGI on Ctrl+C without contending for
    // stdin_loop's &mut connection.
    let signal_connection_for_ctrlc = match &signal_connection {
        Some(connection) => Some(
            connection
                .try_clone()
                .wrap_err("Failed to clone signal channel connection for Ctrl+C handling")?,
        ),
        None => None,
    };

    match signal_connection {
        Some(signal_connection) => {
            stdin_loop::start(signal_connection, action_loop.get_stop_signal());
        }
        None => debug!("Stdin forwarding disabled (--no-signal)"),
    }

    signal_loop::start(
        action_loop.get_channel(),
        action_loop.get_stop_signal(),
        signal_connection_for_ctrlc,
    )?;

    let exit = action_loop.run();

    match exit {
        ExitAction::CleanExit(code) => {
            process.stop(config.kill);
            debug!("Exiting G-CLI with exit code {}", code);
            Ok(code)
        }
        ExitAction::ForcedExit => {
            debug!(
                "Recieved a signal to kill the process. LabVIEW has been notified via the signal channel; \
                 waiting up to {:?} before force-killing (--ctrlc-timeout)",
                config.ctrlc_timeout
            );
            process.stop(config.ctrlc_timeout);
            Ok(-1)
        }
    }
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
        .set_time_format_custom(format_description!(
            "[hour]:[minute]:[second].[subsecond digits:3]"
        ));
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
    signal_listener: Option<&AppListener>,
) -> Result<labview::process::MonitoredProcess> {
    let launch_path = config.to_launch.clone();
    let extension_as_str = launch_path.extension().map(|ext| {
        //allow panic here as I don't expect we will ever really hit it.
        ext.to_str().expect("Extension isn't valid UTF-8")
    });
    let signal_port = signal_listener.map(|l| l.port());

    match extension_as_str {
        Some("vi") => {
            let active_install = find_install(&config.lv_version_string, config.bitness)?;

            launch_lv(
                &active_install,
                launch_path,
                app_listener.port(),
                signal_port,
                config.allow_dialogs,
            )
            .wrap_err("Failed to Launch LabVIEW")
        }
        Some("exe") => launch_exe(launch_path, app_listener.port(), signal_port)
            .wrap_err("Failed to Launch Executable"),
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
                signal_port,
                config.allow_dialogs,
            )
            .wrap_err("Failed to launch LabVIEW")
        }
        Some(extension) => Err(eyre!("Unknown extension {:?}", extension)),
    }
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
