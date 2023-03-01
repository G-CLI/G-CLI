# Release Notes

## v3.0.0

### New Features

* Proxy rebuilt in Rust for cross-platform support.
* Standard error can be written to instead of standard out.
* Exit With Error Code.vi now writes to standard error.
* Non-verbose mode does not add anything to the output you generate.
* ClearCache tool included.
* Add Arch Flag for 32 bit and 64 bit setting
* Support for launching VIs in packed libraries/LLBs. (experimental)
* Experimental support for Linux

### Deprecations/Breaking Changes

* `labview-cli` command no longer works.
*  `--no-launch` deprecated.
* `--timeout` deprecated in favour of `--connect-timeout` to remove ambiguity
* Minimum version is now 2015.


## v2.4.0

### New Features

* Detect if LabVIEW allows multiple instances and allow multiple runs of the same VI if it does. (#96, thanks Zieleslaw)

## v2.3.0

### New Features

* Added --kill-timeout option to have seperate wait time before killing labVIEW process.
* Now zero output if verbose mode isn't used.
* Added write line function in library.
* Mass compiles G-CLI tools after install.

## v2.2.0

### New Features

* Added the option to pass an exit code to exit code with error to be used in the no error case.

### Bug Fixes

* Improved some error handling in the G CLI exe.

## v2.1.0

### New Features

* Pass current working directory to the LabVIEW library.
* Added VI to calculate absolute paths from path parameters.
* Added command line parser.

### Bug Fixes

* Fixed exception due to race condition accessing the same process we have closed.

## v2.0.0

### New Features

* Arguments sent over TCP link. First step to not having to launch LabVIEW.
* Port passed through service locator to remove requirement to launch LabVIEW.
* Handles if LabVIEW is already open.