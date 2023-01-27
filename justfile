set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

unit-test:
  g-cli -v --lv-ver 2015 viTester -- "LabVIEW Source\G CLI.lvproj" -xml "lv-results.xml"
  
integration-test:
  pwsh -File "./run_integration_tests.ps1" ./rust-proxy/target/release/