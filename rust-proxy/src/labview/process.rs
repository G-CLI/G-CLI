use log::{debug, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
use super::{Registration, error::LabVIEWError};

type Pid = i32;

pub struct MonitoredProcess {
    stop_channel: mpsc::Sender<bool>,
    /// Port registration for management
    /// Im not totally convinced this is the right place for it.
    port_registration: Option<Registration>
}

impl MonitoredProcess {
    pub fn start(path: PathBuf, args: &[String], port_registration: Option<Registration>) -> Result<Self, LabVIEWError> {
        let original_pid = launch(&path, args)?;

        //setup a channel for passing stop messages//
        let (stop_tx, stop_rx) = mpsc::channel();

        let thread_path = path.clone();

        thread::spawn(move || {
            let mut current_pid = original_pid;

            while stop_rx.try_recv().is_err() {
                let matching_processes = find_instances(&thread_path);

                if let Some(id) = find_process(&matching_processes, current_pid) {
                    if id != current_pid {
                        current_pid = id;
                        info!("Process lost + found at PID {}", id);
                    }
                } else {
                    info!("Process Lost");
                    break;
                }

                thread::sleep(Duration::from_millis(100));
            }
        });

        Ok(Self {
            stop_channel: stop_tx,
            port_registration
        })
    }

    /// Send a stop command to the monitoring thread.
    pub fn stop_monitor(&self) {
        self.stop_channel.send(true).unwrap();
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

/// Launches the LabVIEW process.
/// Returns the process ID.
fn launch(path: &PathBuf, args: &[String]) -> Result<Pid, LabVIEWError> {
    let launch_result = Command::new(path).args(args).spawn();

    match launch_result {
        Ok(output) => {
            debug!("Process launched with PID {}", output.id());
            return Ok(output.id() as Pid);
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

    for (pid, process) in sys.get_processes() {
        let process_path = process.exe();
        if process_path == path {
            processes.insert(pid.clone() as Pid, process.name().to_owned());
        }
    }

    processes
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
        processes.insert(1, String::from("Process"));
        return processes;
    }

    #[test]
    fn find_processes_same_process() {
        let processes = test_process_list();

        assert_eq!(find_process(&processes, 1), Some(1));
    }

    #[test]
    fn find_processes_new_process() {
        let processes = test_process_list();

        assert_eq!(find_process(&processes, 2), Some(1));
    }

    #[test]
    fn find_processes_none() {
        let mut processes = test_process_list();
        processes.remove(&1); //remove the only entry.

        assert_eq!(find_process(&processes, 1), None);
    }
}
