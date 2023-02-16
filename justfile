set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

g_cli_args := " -v --lv-ver 2015"
lv_proj := "\"LabVIEW Source\\G CLI.lvproj\""

unit-test:
  g-cli {{g_cli_args}} viTester -- {{lv_proj}} -xml "lv-results.xml"
  
build-integration-test:
  g-cli {{g_cli_args}} lvBuild -- {{lv_proj}} "CWD Test"
  g-cli {{g_cli_args}} lvBuild -- {{lv_proj}} "Echo Test"
  g-cli {{g_cli_args}} lvBuild -- {{lv_proj}} "Large Output Test"
  g-cli {{g_cli_args}} lvBuild -- {{lv_proj}} "Quit with Code Test"
  g-cli {{g_cli_args}} lvBuild -- {{lv_proj}} "Test In Packed Library"
  
integration-test:
  pwsh -File "./run_integration_tests.ps1" ./rust-proxy/target/release/
  
integration-test-with-build: build-integration-test integration-test 
