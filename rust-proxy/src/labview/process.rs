use log::{debug, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};

type Pid = i32;

pub struct MonitoredProcess {
    path: PathBuf,
    stop_channel: mpsc::Sender<bool>,
    process_lost_channel: mpsc::Receiver<bool>,
}

impl MonitoredProcess {
    pub fn start(path: PathBuf) -> Self {
        let original_pid = launch(&path);

        //setup a channel for passing stop messages/
        let (stop_tx, stop_rx) = mpsc::channel();

        //setup a channel for sending a notification that the process is lost.
        let (lost_tx, lost_rx) = mpsc::channel();

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
                    //send the warning - don't care if this errors.
                    let _ = lost_tx.send(true);
                    break;
                }

                thread::sleep(Duration::from_millis(100));
            }
        });

        Self {
            path,
            stop_channel: stop_tx,
            process_lost_channel: lost_rx,
        }
    }

    /// Send a stop command to the monitoring thread.
    pub fn stop(&self) {
        self.stop_channel.send(true);
    }

    /// Check on the process lost signal.
    /// TODO: This only returns true once. It should be changed to always return true after process lost.
    pub fn check_process_stopped(&self) -> bool {
        self.process_lost_channel.try_recv().is_ok()
    }
}

/// Launches the LabVIEW process.
/// Returns the process ID.
fn launch(path: &PathBuf) -> Pid {
    let output = Command::new(path).spawn().expect("Failed to launch");

    debug!("Process launched with PID {}", output.id());

    output.id() as i32
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
