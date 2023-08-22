mod common;

use common::{g_cli_args, integration_build_path, integration_vi_path};

use snapbox::cmd::cargo_bin;
use snapbox::cmd::Command;

#[test]
fn test_echo_parameters() {
    let vi = integration_vi_path("Echo Parameters.vi");
    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["Param1", "Param2"]))
        .assert()
        .stdout_matches("Param1\tParam2[..]")
        .code(0);
}

#[test]
fn test_echo_working_dir() {
    let vi = integration_vi_path("Echo CWD.vi");
    let cwd = std::env::current_dir().unwrap();
    //for some reason slashes need reversing.
    let cwd_str = cwd.to_string_lossy().into_owned().replace("\\", "/");

    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["Param1", "Param2"]))
        .assert()
        .stdout_matches(cwd_str);
}

// Ignored currently due to regression.
#[test]
#[ignore = "Regression To Be Fixed"]
fn test_echo_working_dir_ppl() {
    let vi = integration_build_path("Tests.lvlibp/cwd.vi");
    let cwd = std::env::current_dir().unwrap();
    //for some reason slashes need reversing.
    let cwd_str = cwd.to_string_lossy().into_owned().replace("\\", "/");

    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["Param1", "Param2"]))
        .assert()
        .stdout_matches(cwd_str);
}

#[test]
fn test_large_output() {
    let vi = integration_vi_path("Generate Large Output.vi");
    println!("{:?}", std::env::current_dir().unwrap());
    let run = Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["10000"]))
        .output()
        .unwrap();

    assert_eq!(run.stdout.len(), 10000);
    assert_eq!(run.stderr.len(), 0);
    assert_eq!(run.status.code().unwrap(), 0);
}

#[test]
fn test_large_error() {
    let vi = integration_vi_path("Generate Large Error.vi");
    let run = Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["10000"]))
        .output()
        .unwrap();

    assert_eq!(run.stderr.len(), 10000);
    assert_eq!(run.stdout.len(), 0);
    assert_eq!(run.status.code().unwrap(), 0);
}

#[test]
fn test_exit_code() {
    let vi = integration_vi_path("Quit With Parameter Code.vi");

    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["100"]))
        .assert()
        .code(100);
}

#[cfg(target_os = "windows")]
#[test]
fn test_negative_exit_code() {
    let vi = integration_vi_path("Quit With Parameter Code.vi");

    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["-10000"]))
        .assert()
        .code(-10000);
}

#[test]
fn test_unicode_in_params_and_response() {
    let vi = integration_vi_path("Check Unicode Response.vi");

    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["HÜll°"]))
        .assert()
        .stdout_matches("HÜll°[..]");
}

#[test]
fn test_unicode_in_filename() {
    let vi = integration_vi_path("Check Unicode Response HÜll°.vi");

    Command::new(cargo_bin("g-cli"))
        .args(g_cli_args(&vi, ["HÜll°"]))
        .assert()
        .stdout_matches("HÜll°[..]");
}
