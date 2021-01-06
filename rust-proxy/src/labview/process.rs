use std::collections::HashMap;
use std::process::Command;
use sysinfo::{System, SystemExt, ProcessExt};



const COMMAND: &str = "firefox";

/// Launches the LabVIEW process.
/// Returns the process ID.
pub fn launch() -> u32 {

    let output = Command::new(COMMAND)
        .spawn().expect("Failed to launch");

    output.id()
}

/// Returns a list of all instances running of LabVIEW
pub fn find_instances() -> HashMap<i32, String> {

    let sys = System::new_all();
    let mut processes = HashMap::new();

    for (pid, process) in sys.get_processes() {
        if process.name() == COMMAND {
            processes.insert(pid.clone(), process.name().to_owned());
        }
    }

    processes
}
