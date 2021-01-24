mod labview;

use labview::detect_installations;
use simple_logger::SimpleLogger;
use std::path::PathBuf;

fn main() {
    SimpleLogger::new().init().unwrap();

    detect_installations();

    let monitor = labview::process::MonitoredProcess::start(PathBuf::from(
        "C:\\Program Files (x86)\\National Instruments\\LabVIEW 2011\\LabVIEW.exe",
    ));

    std::thread::sleep(std::time::Duration::from_millis(10000));

    print!("{}", monitor.check_process_stopped());
    print!("{}", monitor.check_process_stopped());

    monitor.stop();
}
