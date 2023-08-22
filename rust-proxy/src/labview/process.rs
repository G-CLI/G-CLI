use super::{error::LabVIEWError, Registration};
use log::{debug, info};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

const POLL_INTERVAL: Duration = Duration::from_millis(1000);

// TODO: There are definately improvements to the process monitoring. For example reusing the system item.

pub struct MonitoredProcess {
    stop_channel: mpsc::Sender<Option<Duration>>,
    /// Port registration for management
    /// Im not totally convinced this is the right place for it.
    port_registration: Option<Registration>,
    monitor_thread: JoinHandle<()>,
}

impl MonitoredProcess {
    pub fn start(
        path: PathBuf,
        args: &[OsString],
        port_registration: Option<Registration>,
    ) -> Result<Self, LabVIEWError> {
        let original_pid = launch(&path, args)?;

        //setup a channel for passing stop messages//
        let (stop_tx, stop_rx) = mpsc::channel::<Option<Duration>>();

        let thread_path = path;

        let monitor_thread = std::thread::Builder::new()
            .name("Process Monitor".to_string())
            .spawn(move || {
                let mut current_pid = Some(Pid::from_u32(original_pid));

                // Loop until we recieve a stop. The only way to leave is when the main thread has sent stop.
                // if we stop independently we get a race condition where the main loop will send stop to an invalid channel.
                // Wrap the PID in the option where None means we have lost the process to gate on the kill process.
                loop {
                    match stop_rx.try_recv() {
                        Ok(kill) => {
                            //stop requested. See if we have been asked to kill the process.
                            //disable if we aren't tracking a process though.
                            if let Some(pid) = current_pid {
                                kill_process_with_timeout(kill, &thread_path, pid)
                            };
                            debug!(
                                "Stopping LabVIEW monitoring due to stop command from application"
                            );
                            break;
                        }

                        Err(_) => {
                            //no stop command. Validate processes if we are still monitoring a pid.
                            if let Some(pid) = current_pid {
                                if let Some(id) = check_process(&thread_path, pid) {
                                    current_pid = Some(id);
                                } else {
                                    debug!("The LabVIEW process appears to have closed down");
                                    current_pid = None;
                                }
                            }
                        }
                    }

                    sleep(POLL_INTERVAL);
                }
                debug!("Monitoring thread completed");
            })
            .expect("Could not create monitor thread.");

        Ok(Self {
            stop_channel: stop_tx,
            port_registration,
            monitor_thread,
        })
    }

    /// Send a stop command to the monitoring thread and blocks until complete.
    ///
    /// * `kill_process` - Set to None to leave the process running or provide a timeout for when the process should be killed if it is still active.
    pub fn stop(self, kill_process: Option<Duration>) {
        //todo: error handling
        self.stop_channel.send(kill_process).unwrap();
        debug!("Waiting on monitoring thread to complete");
        self.monitor_thread.join().unwrap();
    }

    /// Registers that the comms are connected so any action required can be taken like cancelling service discovery.
    pub fn set_connected(&mut self) -> Result<(), LabVIEWError> {
        // We will consume the registration so take it out of the monitor.
        let port_registration = self.port_registration.take();

        if let Some(registration) = port_registration {
            registration.unregister()?;
        }

        Ok(())
    }
}

/// If the kill option is set then it will give the process that duration to stop on it's own.
/// After that timeout, it will use the kill command.
fn kill_process_with_timeout(kill_option: Option<Duration>, thread_path: &Path, pid: Pid) {
    if let Some(timeout) = kill_option {
        info!(
            "Forcing LabVIEW to terminate in {}ms if it doesn't close.",
            timeout.as_millis()
        );
        let end_time = Instant::now() + timeout;

        loop {
            let process_closed = check_process(thread_path, pid).is_none();
            let timeout_passed = Instant::now() > end_time;
            if process_closed {
                break;
            } else if timeout_passed {
                //kill the process.
                kill(pid);
                break;
            } else {
                sleep(POLL_INTERVAL);
            }
        }
    } else {
        debug!("Monitoring complete and kill not requested.");
    }
}

/// Checks if the process is still running and returns the new PID if it is.
fn check_process(thread_path: &Path, current_pid: Pid) -> Option<Pid> {
    let matching_processes = find_instances(thread_path);
    let process_result = find_process(&matching_processes, current_pid);
    if let Some(id) = process_result {
        if id != current_pid {
            info!("Process lost + found at PID {}", id);
        }
    } else {
        info!("Process Lost");
    }
    process_result
}

/// Launches the LabVIEW process.
/// Returns the process ID.
#[cfg(not(target_os = "windows"))]
fn launch(path: &Path, args: &[OsString]) -> Result<u32, LabVIEWError> {
    use std::process::{Command, Stdio};
    //map stdin, out and err to null to prevent holding this process open.

    let mut command = Command::new(path);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let launch_result = command.spawn();

    match launch_result {
        Ok(output) => {
            debug!("Process launched with PID {}", output.id());
            Ok(output.id())
        }
        Err(e) => Err(LabVIEWError::ProcessLaunchFailed(e)),
    }
}

/// This module takes utility functions from the std library of rust
/// that we lost by bypassing the standard process module.
///
/// These are reused here under the MIT license with modifications
/// due to it being outside the std library.
#[cfg(target_os = "windows")]
mod process_utilities {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStrExt;
    use std::result::Result;
    use std::{ffi::OsStr, path::Path};

    use crate::labview::error::LabVIEWError;

    fn ensure_no_nuls<T: AsRef<OsStr>>(str: T) -> Result<T, LabVIEWError> {
        if str.as_ref().encode_wide().any(|b| b == 0) {
            Err(LabVIEWError::NullCharInArgument)
        } else {
            Ok(str)
        }
    }

    // Produces a wide string *without terminating null*; returns an error if
    // `prog` or any of the `args` contain a nul.
    pub fn make_command_line(
        prog: &Path,
        args: &[OsString],
    ) -> std::result::Result<Vec<u16>, LabVIEWError> {
        // Encode the command and arguments in a command line string such
        // that the spawned process may recover them using CommandLineToArgvW.
        let mut cmd: Vec<u16> = Vec::new();

        // CreateFileW has special handling for .bat and .cmd files, which means we
        // need to add an extra pair of quotes surrounding the whole command line
        // so they are properly passed on to the script.
        // See issue #91991.
        let is_batch_file = prog
            .extension()
            .map(|ext| ext.eq_ignore_ascii_case("cmd") || ext.eq_ignore_ascii_case("bat"))
            .unwrap_or(false);
        if is_batch_file {
            cmd.push(b'"' as u16);
        }

        // Always quote the program name so CreateProcess doesn't interpret args as
        // part of the name if the binary wasn't found first time.
        append_arg(&mut cmd, prog.as_os_str())?;
        for arg in args {
            cmd.push(' ' as u16);
            append_arg(&mut cmd, arg)?;
        }
        if is_batch_file {
            cmd.push(b'"' as u16);
        }
        return Ok(cmd);

        fn append_arg(cmd: &mut Vec<u16>, arg: &OsStr) -> std::result::Result<(), LabVIEWError> {
            // If an argument has 0 characters then we need to quote it to ensure
            // that it actually gets passed through on the command line or otherwise
            // it will be dropped entirely when parsed on the other end.
            ensure_no_nuls(arg)?;
            let (quote, escape) = (true, true);
            if quote {
                cmd.push('"' as u16);
            }

            let mut backslashes: usize = 0;
            for x in arg.encode_wide() {
                if escape {
                    if x == '\\' as u16 {
                        backslashes += 1;
                    } else {
                        if x == '"' as u16 {
                            // Add n+1 backslashes to total 2n+1 before internal '"'.
                            cmd.extend((0..=backslashes).map(|_| '\\' as u16));
                        }
                        backslashes = 0;
                    }
                }
                cmd.push(x);
            }

            if quote {
                // Add n backslashes to total 2n before ending '"'.
                cmd.extend((0..backslashes).map(|_| '\\' as u16));
                cmd.push('"' as u16);
            }
            Ok(())
        }
    }
}

/// Launches the LabVIEW process.
/// Returns the process ID.
/// This is a specialised version using the windows API to avoid handle inheritance.
#[cfg(target_os = "windows")]
fn launch(path: &Path, args: &[OsString]) -> Result<u32, LabVIEWError> {
    use std::ptr;
    use windows::core::{PCWSTR, PWSTR};
    use windows::Win32::{
        Foundation::CloseHandle,
        System::Threading::{
            CreateProcessW, CREATE_NEW_PROCESS_GROUP, CREATE_UNICODE_ENVIRONMENT, DETACHED_PROCESS,
            PROCESS_INFORMATION, STARTUPINFOW,
        },
    };

    let mut pi = PROCESS_INFORMATION::default();
    let si = STARTUPINFOW::default();

    //build out required command line.
    let mut command = process_utilities::make_command_line(path, args)?;

    //app name not required - build it into command line.
    let dwcreationflags = CREATE_UNICODE_ENVIRONMENT | DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP;
    let success = unsafe {
        CreateProcessW(
            PCWSTR(ptr::null()),
            PWSTR(command.as_mut_ptr()),
            None,
            None,
            false,
            dwcreationflags,
            None,
            PCWSTR(ptr::null()),
            &si,
            &mut pi as *mut PROCESS_INFORMATION,
        )
    };

    if success.is_ok() {
        let pid = pi.dwProcessId;
        unsafe {
            let _ = CloseHandle(pi.hProcess);
            let _ = CloseHandle(pi.hThread);
        }
        debug!("Process launched with PID {}", pid);
        Ok(pid)
    } else {
        Err(LabVIEWError::ProcessLaunchFailed(
            std::io::Error::last_os_error(),
        ))
    }
}

/// Returns a list of all instances running of LabVIEW
fn find_instances(path: &Path) -> HashMap<Pid, String> {
    let sys = System::new_all();
    let mut processes = HashMap::new();

    for (pid, process) in sys.processes() {
        let process_path = process.exe();
        if process_path == path {
            processes.insert(*pid, process.name().to_owned());
        }
    }

    processes
}

/// Kill the process by PID
fn kill(pid: Pid) {
    info!("Killing LabVIEW process ({})", pid);
    let sys = System::new_all();
    if let Some(process) = sys.process(pid) {
        process.kill();
    } else {
        info!("Process ID Not Found to kill. It may have already closed.");
    }
}

/// Find the process in the list.
/// Return Some(pid) if a valid process is found.
/// Return None if no process matches.
fn find_process(processes: &HashMap<Pid, String>, original_id: Pid) -> Option<Pid> {
    let original_found = processes.contains_key(&original_id);

    if original_found {
        Some(original_id)
    } else {
        // Returns None if no others, or some if they do.
        processes.keys().next().copied()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_process_list() -> HashMap<Pid, String> {
        let mut processes = HashMap::new();
        processes.insert(Pid::from(1), String::from("Process"));
        return processes;
    }

    #[test]
    fn find_processes_same_process() {
        let processes = test_process_list();

        assert_eq!(find_process(&processes, Pid::from(1)), Some(Pid::from(1)));
    }

    #[test]
    fn find_processes_new_process() {
        let processes = test_process_list();

        assert_eq!(find_process(&processes, Pid::from(2)), Some(Pid::from(1)));
    }

    #[test]
    fn find_processes_none() {
        let mut processes = test_process_list();
        processes.remove(&Pid::from(1)); //remove the only entry.

        assert_eq!(find_process(&processes, Pid::from(1)), None);
    }
}
