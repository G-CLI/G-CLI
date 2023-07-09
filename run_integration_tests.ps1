#Making this explicit as it is important to how the script runs.
$ErrorActionPreference = 'stop'


$delay_between_tests = 3

# directory with the built g-cli binary is the first argument
$cli_cmd = $args[0] + 'g-cli'
# arguments to pass to g-cli is second argument -- convert to array (space delimited)
$common_params = -split $args[1]
# looks like --> ./rust-proxy/target/release/g-cli -v --lv-ver 2020 --x64
Write-Output "$cli_cmd $common_params"

Write-Output "Echo Parameters"
$output_matches = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Echo Parameters.vi" -- "Param 1" "Param 2" | % { $_.Trim() -eq "Param 1	Param 2" }
if (!$output_matches) { 
  Write-Output "Echo Parameters VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests

Write-Output "Echo CWD"
$output_matches = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Echo CWD.vi"  | % { $_.Trim() -eq $pwd.Path }
if (!$output_matches) { 
  Write-Output "Echo CWD VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests

Write-Output "lvlibp Echo CWD"
##$output_matches = & "$cli_cmd" "Builds/Tests.lvlibp/cwd.vi" | %{$_.Trim() -eq $pwd.Path }
##if(!$output_matches) { 
##  Write-Output "Echo CWD VI Failed"
##  Exit 1
## }
##Start-Sleep -s $delay_between_tests
Write-Output "WARNING: Skipping packed library test due to build issue"

Write-Output "Large Output"
& "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Generate Large Output.vi" -- 10000
if (!$?) { 
  Write-Output "Large Output VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests

Write-Output "Large Output Error"
$ErrorActionPreference = 'continue'
$output = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Generate Large Error.vi" -- 10000 2>&1
$errors = $output | Where-Object { $_.gettype().Name -eq "ErrorRecord" }
Write-Host "STDERR"
Write-Host $errors
if (!$errors) { 
  Write-Output "Nothing in Error Output"
  Exit 1
}
Start-Sleep -s $delay_between_tests
$ErrorActionPreference = 'stop'


& "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Quit With Parameter Code.vi" -- 100
Write-Output "Exit Code $LastExitCode"
if ($LastExitCode -ne 100) {
  Write-Output "Quit with Code VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests

if ($IsWindows) {
  & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Quit With Parameter Code.vi" -- -10000
  Write-Output "Exit Code $LastExitCode"
  if ($LastExitCode -ne -10000) {
    Write-Output "Quit with Negative Code VI Failed"
    Exit 1
  }
  Start-Sleep -s $delay_between_tests
}

# Currently have a regression with these cases.
if ($IsWindows) {
  $output_matches = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Check Unicode Response.vi" -- "HÜll°"  | ForEach-Object { $_.Trim() -eq "HÜll°" }
  if (!$output_matches) { 
    Write-Output "Non-Ascii in Input/Output Failed"
    Exit 1
  }
  Start-Sleep -s $delay_between_tests

  $output_matches = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Check Unicode Response HÜll°.vi" -- "HÜll°"   | ForEach-Object { $_.Trim() -eq "HÜll°" }
  if (!$output_matches) { 
    Write-Output "Non-Ascii in Name Failed"
    Exit 1
  }
  Start-Sleep -s $delay_between_tests
}

$output_matches = & "$cli_cmd" $common_params "Builds/Echo CLI.exe" -- "Param 1" "Param 2" | ForEach-Object { $_.Trim() -eq "Param 1	Param 2" }
if (!$output_matches) { 
  Write-Output "Echo Parameters EXE Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests 



$output_matches = & "$cli_cmd" $common_params "Builds/Echo CWD.exe" | ForEach-Object { $_.Trim() -eq $pwd.Path }
if (!$output_matches) { 
  Write-Output "Echo CWD EXE Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params "Builds/LargeOutput.exe" -- 10000
if (!$?) { 
  Write-Output "Large Output EXE Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests

& "$cli_cmd" $common_params "Builds/QuitWithCode.exe" -- 100
if ($LastExitCode -ne 100) {
  Write-Output "Quit with Code EXE Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


Write-Output "All Tests Completed Successfully"
Exit 0

