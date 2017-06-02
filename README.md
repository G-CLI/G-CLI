# LabVIEW-CLI
A proxy mechanism allow LabVIEW programs to easily write out to the command line.

## Installing

The completed package will be available via the releases tab on Github and will be in the LabVIEW tools network soon!

To install download the VI package (or search it on the tools network) and run. This will install the LabVIEW library and the required Windows application.

The Windows application has to be added to the system path so it will cause a UAC prompt in Windows.

The VI package is built for LabVIEW 2011 and later. The Windows application has been tested on Windows 7 & 10 but should work on other versions of Windows.

## Launching

When installed with the supplied installer the application is added to Windows path variable so you can launch it from any command line using `labview-cli`

## Command Line Arguments

`labview-cli [options] launch-vi [-- application arguments]`

last argument (or last before -- if user arguments) is what to launch.

--version: Prints the program version and exits

-v: Verbose mode.

--no-launch: Doesn't launch anything automatically, you must run your software manually. Overrides --timeout to -1.

--lv-exe: LabVIEW Executable to use. Defaults to LV 2014 32 bit default path at the minute.

--lv-ver: LabVIEW version to use instead of specifying execution e.g. 2015

--x64: Launch in 64 bit LabVIEW

--timeout: Maximum time (in ms) to wait for the LabVIEW program to connect to the CLI (-1 = Infinity).

--kill: Forces the LabVIEW process to exit after the CLI receives a return code/error

Any arguments after -- are passed to the application.
