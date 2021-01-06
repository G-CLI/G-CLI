mod labview;

fn main() {
    labview::process::launch();

    let processes = labview::process::find_instances();

    for (pid, name) in processes {
        println!("[{}] {}", pid, name);
    }
}
