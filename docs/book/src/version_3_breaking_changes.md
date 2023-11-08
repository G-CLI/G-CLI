This page is an easy way to track and share the expected breaking changes as part of version 3.0.

## v3.0 New Features

* Standard Error output is now available in write functions.
* ClearCache tool included in the install.
* Support for packed libraries and LLBs.

## System Requirements

These are mostly driven by moving the g-cli tool to Rust for cross platform compatibility.

Mostly, there are just about modernising the minimum requirements and I don't expect a large knock-on affect.

1. 64 Bit OS Only - The tool will still work with 32 bit LabVIEW but the underlying operating system will have to be 64 bit.
2. Minimum LabVIEW version of 2015 (from 2011). This represents a similar age from the original release and I expect covers most users. It allows the use of VI Tester reporting in the CI toolchain which is currently limited to 2013.

## Tool Interface

### Output Changes

I want to use this as a opportunity to hide all G-CLI output by default. This has been requested before and seems like a sensible request. A major version change seems a good time to introduce it.

* Default output - only errors.
* -v flag - verbose output - current verbose.

### Deprecate Timeout

The `--timeout` flag is now ambiguous with `--kill-timeout`. This will be deprecated in favour of a `--connect-timeout` flag.

### Deprecate `labview-cli` shim

The shim to still allow the old name to be removed.

### Deprecate No Launch

Its unclear what the use of this is now. All use cases I can think of would launch the app and this increases the software complexity so unless there are significant requirements this will be deprecated as well.

### Deprecate/Alter 64 bit flag

Right now we set --x64 to request 64 bit but this not clear by its absence. This flag will continue to work but we will add a new flag for --arch 64 or --arch 32. No flags will default to 32 bit to maintain current behaviour.

### StdErr Output

The default case for the write functions is still to use StdOut.

However Exit with Error Code will now write the error message to StdErr. This could lead to errors being shown for the command where they weren't before.