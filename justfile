set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]


integration-test:
  pwsh -File "./run_integration_tests.ps1" ./rust-proxy/target/release/