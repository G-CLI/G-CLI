# Contributing

## General Guidance

If there is a change you would like first create an issue in Github so we can discuss what you need and the impact.

Then fork the repo and put in a pull request for your changes. Before including the pull request please ensure:

* You include unit tests where appropriate (we are trying to improve the current unit testing).
* The project builds and the tests pass.

Don't worry about including a final build - we have a Jenkins server configured to automate the build so we can include that in the next release.

## Proxy Code - Rust 

The rust code is held under the rust-proxy folder. It consists of a single binary crate and wix configuration for the installer.

The usual `cargo test` and `cargo build` commands will work for development.

To build the installer you must install cargo wix. This is done as part of the CI system though and isn't necessary for general development.


## LabVIEW

The LabVIEW library is maintained in LabVIEW 2015 32-bit and the source is in the LabVIEW Source subfolder.

VI Tester is used for unit testing in LabVIEW.

As LabVIEW for Linux 2015 is not readily accessible all development changes need to be done on Windows then tested on Linux.

### Distributing

There is a vipc file next to the project for building the output package.

## Just Command Runner

This project uses the Just Command Runner (https://github.com/casey/just/tree/master) to help automate some of the tasks. I recommend you follow the installation instructions there.

The current commands are:

* **just unit-test**: Run the unit tests through VI Tester
* **just integration-test**: Probably the most useful. Run the integration test against the existing builds.
* **just integration-test-with-build**: Runs the build for the integration tests then runs them. Not tested across all platforms.


For a complete release test this should be tested with LabVIEW's allowMultipleInstances flag true and false. For a quick test just false is required.
