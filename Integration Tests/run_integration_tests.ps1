

$common_params='-v'
$delay_between_tests=3
$cli_cmd= $args[0] + 'g-cli'
echo $cli_cmd $common_params

& "$cli_cmd" -v "Echo Parameters.vi" -- "Param 1" "Param 2" | find "Param 1	Param 2"
if(!$?) { 
  echo "Echo Parameters VI Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

& "$cli_cmd" -v "Echo CWD.vi" | find $pwd
if(!$?) { 
  echo "Echo CWD VI Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

& "$cli_cmd" $common_params "Generate Large Output.vi" -- 10000
if(!$?) { 
  echo "Large Output VI Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

$output = & "$cli_cmd" $common_params "Generate Large Error.vi" -- 10000 2>&1
$errors = $output | ?{$_.gettype().Name -eq "ErrorRecord"}
Write-Host "STDERR"
Write-Host $errors
if(!$errors) { 
  echo "Nothing in Error Output"
  Exit 1
 }
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params "Quit With Parameter Code.vi" -- 10000
echo "Exit Code $LastExitCode"
if ($LastExitCode -ne 10000) {
  echo "Quit with Code VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params "Quit With Parameter Code.vi" -- -10000
echo "Exit Code $LastExitCode"
if ($LastExitCode -ne -10000) {
  echo "Quit with Negative Code VI Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params ".\exes\Echo CLI.exe" -- "Param 1" "Param 2" | find "Param 1	Param 2"
if(!$?) { 
  echo "Echo Parameters EXE Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests 


& "$cli_cmd" $common_params ".\exes\Echo CWD.exe" | find $pwd
if(!$?) { 
  echo "Echo CWD EXE Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests


& "$cli_cmd" $common_params ".\exes\LargeOutput.exe" -- 10000
if(!$?) { 
  echo "Large Output EXE Failed"
  Exit 1
 }
Start-Sleep -s $delay_between_tests

& "$cli_cmd" $common_params ".\exes\QuitWithCode.exe" -- 10000
if ($LastExitCode -ne 10000) {
  Echo "Quit with Code EXE Failed"
  Exit 1
}
Start-Sleep -s $delay_between_tests


echo "All Tests Completed Successfully"
Exit 0

