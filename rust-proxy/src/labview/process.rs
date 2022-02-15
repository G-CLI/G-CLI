use super::{error::LabVIEWError, Registration};
use log::{debug, error, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

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
        args: &[String],
        port_registration: Option<Registration>,
    ) -> Result<Self, LabVIEWError> {
        let original_pid = launch(&path, args)?;

        //setup a channel for passing stop messages//
        let (stop_tx, stop_rx) = mpsc::channel::<Option<Duration>>();

        let thread_path = path.clone();

        let monitor_thread = spawn(move || {
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
                        debug!("Stopping monitoring due to stop command from application");
                        break;
                    }

                    Err(_) => {
                        //no stop command. Validate processes if we are still monitoring a pid.
                        if let Some(pid) = current_pid {
                            if let Some(id) = check_process(&thread_path, pid) {
                                current_pid = Some(id);
                            } else {
                                debug!("The process appears to have closed down.");
                                current_pid = None;
                            }
                        }
                    }
                }

                sleep(POLL_INTERVAL);
            }
        });

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
fn kill_process_with_timeout(kill_option: Option<Duration>, thread_path: &PathBuf, pid: Pid) {
    if let Some(timeout) = kill_option {
        info!(
            "Process Kill Requested - Monitoring for Timeout {}ms",
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
fn check_process(thread_path: &PathBuf, current_pid: Pid) -> Option<Pid> {
    let matching_processes = find_instances(thread_path);
    let process_result = find_process(&matching_processes, current_pid);
    if let Some(id) = process_result {
        if id != current_pid {
            info!("Process lost + found at PID {}", id);
        }
    } else {
        info!("Process Lost");
    }
    return process_result;
}

/// Launches the LabVIEW process.
/// Returns the process ID.
fn launch(path: &PathBuf, args: &[String]) -> Result<u32, LabVIEWError> {
    let launch_result = Command::new(path).args(args).spawn();

    match launch_result {
        Ok(output) => {
            debug!("Process launched with PID {}", output.id());
            return Ok(output.id());
        }
        Err(e) => {
            return Err(LabVIEWError::ProcessLaunchFailed(e));
        }
    }
}

/// Returns a list of all instances running of LabVIEW
fn find_instances(path: &PathBuf) -> HashMap<Pid, String> {
    let sys = System::new_all();
    let mut processes = HashMap::new();

    for (pid, process) in sys.processes() {
        let process_path = process.exe();
        if process_path == path {
            processes.insert(pid.clone(), process.name().to_owned());
        }
    }

    processes
}

/// Kill the process by PID
fn kill(pid: Pid) {
    info!("Killing process {}", pid);
    let sys = System::new_all();
    if let Some(process) = sys.process(pid) {
        process.kill();
    } else {
        error!("Process ID Not Found to Kill");
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
        processes
            .keys()
            .next()
            //map to make it owned.
            .map(|id| id.clone())
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
