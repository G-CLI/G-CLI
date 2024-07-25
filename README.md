# G-CLI
A proxy mechanism allowing LabVIEW programs to easily write out to the command line.

## Installing

The latest package is available via the [releases tab on Github](https://github.com/JamesMc86/G-CLI/releases) and is also available on the [NI Tools Network feed on VI Package Manager](vipm://wiresmith_technology_lib_g_cli?repo_url=http://ftp.ni.com/evaluation/labview/lvtn/vipm)

To install download the VI package (or search it on the tools network) and run. This will install the LabVIEW library and the required Windows application.

The Windows application has to be added to the system path so it will cause a UAC prompt in Windows.

The VI package is built for LabVIEW 2015 and later. The Windows application has been tested on Windows 7 & 10 but should work on other versions of Windows.

## Launching

When installed with the supplied installer the application is added to Windows path variable so you can launch it from any command line using `g-cli`

## Usage

See [http://jamesmc86.github.io/G-CLI/](http://g-cli.github.io/G-CLI/) for instructions on using this tool.


## What About The LabVIEW 2018 CLI Tool?

NI now ships a CLI tool in LabVIEW with some common tools for continuous integration.

We have and will continue to develop this as our ability to iterate faster and having something community-driven (aka developed by the people using it) has shown a number of benefits which should show in greater stability and ease of use.

The key benefits to G CLI currently are:

* Supporting libraries for creating tools with custom options and continuous output.
* Automatic detection of installed LV versions (no need to provide long paths or port numbers)
* Support for LabVIEW built executables
