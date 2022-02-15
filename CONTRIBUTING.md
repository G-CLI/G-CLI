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

The LabVIEW library is maintained in LabVIEW 2011 32-bit and the source is in the LabVIEW Source subfolder.

VI Tester is used for unit testing in LabVIEW>

### Distributing

There is a vipc file next to the project for building the output package.

## Integration Testing

The integration tests folder contains a bat file to run a number of tests against LabVIEW VIs and built EXEs. Please run this with your new EXE to test the integration still works.

To update and run the tests:

1. Build and install your updates as a vipb. The tests run against the installed version.
2. Open "Integration Tests/CLI Integration Tests.lvproj" and rebuild all the exes.
3. Close LabVIEW and then run "Integration Tests/Run Integration Tests.bat"
4. The bat file should provide feedback if any tests fail.

For a complete release test this should be tested with LabVIEW's allowMultipleInstances flag true and false. For a quick test just false is required.
