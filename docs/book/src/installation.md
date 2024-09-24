# Installation

There are two components required for G-CLI:

* The G-CLI proxy application which is launched from the command line.
* The G-CLI library which must be installed to each LabVIEW version.

## G-CLI Library

The G-CLI library is available through VIPM.

1. Launch VIPM.
2. Search for `G CLI`
3. Install for each version of LabVIEW you need.

Alternative releases will be listed at https://github.com/JamesMc86/G-CLI/releases.

## G-CLI Proxy

On Windows this is installed by VIPM at the same time as the library.

On Linux you need to download a package from the github releases page at https://github.com/JamesMc86/G-CLI/releases.

## Post Install

On Windows G-CLI must be added to the path so you must restart any terminals or build agents after the first install to include the new path variable.


