#Making this explicit as it is important to how the script runs.
$ErrorActionPreference = 'stop'


$delay_between_tests=3
$cli_cmd= $args[0] + 'g-cli'
echo $cli_cmd 

echo "Echo Parameters"
$matches = & "$cli_cmd" "LabVIEW Source/integration-tests/Echo Parameters.vi" -- "Param 1" "Param 2" | %{$_.Trim() -eq "Param 1	Param 2" }
if(!$matches) { 
  echo "Echo Parameters VI Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

echo "Echo CWD"
$matches = & "$cli_cmd" "LabVIEW Source/integration-tests/Echo CWD.vi"  | %{$_.Trim() -eq $pwd.Path }
if(!$matches) { 
  echo "Echo CWD VI Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

echo "lvlibp Echo CWD"
##$matches = & "$cli_cmd" "Builds/Tests.lvlibp/cwd.vi" | %{$_.Trim() -eq $pwd.Path }
##if(!$matches) { 
##  echo "Echo CWD VI Failed"
##  Exit 1
## }
##Start-Sleep -s $delay_between_tests
echo "WARNING: Skipping packed library test due to build issue"

echo "Large Output"
& "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Generate Large Output.vi" -- 10000
if(!$?) { 
  echo "Large Output VI Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

echo "Large Output Error"
$ErrorActionPreference = 'continue'
$output = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Generate Large Error.vi" -- 10000 2>&1
$errors = $output | ?{$_.gettype().Name -eq "ErrorRecord"}
Write-Host "STDERR"
Write-Host $errors
if(!$errors) { 
  echo "Nothing in Error Output"
  Exit 1
 }
Start-Sleep -s $delay_between_tests
$ErrorActionPreference = 'stop'


& "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Quit With Parameter Code.vi" -- 10000
echo "Exit Code $LastExitCode"
if ($LastExitCode -ne 10000) {
  echo "Quit with Code VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Quit With Parameter Code.vi" -- -10000
echo "Exit Code $LastExitCode"
if ($LastExitCode -ne -10000) {
  echo "Quit with Negative Code VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


$matches = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Check Unicode Response.vi" -- "HÜll°"  | %{$_.Trim() -eq "HÜll°" }
if(!$matches) { 
  echo "Non-Ascii in Input/Output Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

$matches = & "$cli_cmd" $common_params "LabVIEW Source/integration-tests/Check Unicode Response HÜll°.vi" -- "HÜll°"   | %{$_.Trim() -eq "HÜll°" }
if(!$matches) { 
  echo "Non-Ascii in Name Failed"
  Exit 1
 }
 
Start-Sleep -s $delay_between_tests
$matches = & "$cli_cmd" $common_params "Builds\Echo CLI.exe" -- "Param 1" "Param 2" | %{$_.Trim() -eq "Param 1	Param 2" }
if(!$matches) { 
  echo "Echo Parameters EXE Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests 



$matches = & "$cli_cmd" $common_params "Builds\Echo CWD.exe" | %{$_.Trim() -eq $pwd.Path }
if(!$matches) { 
  echo "Echo CWD EXE Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params "Builds\LargeOutput.exe" -- 10000
if(!$?) { 
  echo "Large Output EXE Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

& "$cli_cmd" $common_params "Builds\QuitWithCode.exe" -- 10000
if ($LastExitCode -ne 10000) {
  Echo "Quit with Code EXE Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


echo "All Tests Completed Successfully"
Exit 0

