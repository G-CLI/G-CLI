mod labview;

use log::debug;
use simple_logger::SimpleLogger;

fn main() {

    SimpleLogger::new().init().unwrap();

    let id = labview::process::launch();

    debug!("Process launched at {}", id);

    let processes = labview::process::find_instances();

    for (pid, name) in processes {
        println!("[{}] {}", pid, name);
    }

    let monitor = labview::process::ProcessMonitor::start(id);

    std::thread::sleep(std::time::Duration::from_millis(10000));

    print!("{}", monitor.check_process_stopped());

    monitor.stop();



}
