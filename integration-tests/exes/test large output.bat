
:loop
labview-cli -v "LargeOutput.exe" -- 10000
if %errorlevel% == 0 goto :loop