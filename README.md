# LabVIEW-CLI
A proxy mechanism allow LabVIEW programs to easily write out to the command line.

## Installing

If you select releases above this you should see the v1.0-beta release.

Download the installer to add the Windows component and the vipc file contains the library for LabVIEW development. Or download the source which contains a small demo.

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

Any arguments after -- are passed to the application.
