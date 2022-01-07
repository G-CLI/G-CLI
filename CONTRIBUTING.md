# Contributing

## General Guidance

If there is a change you would like first create an issue in Github so we can discuss what you need and the impact.

Then fork the repo and put in a pull request for your changes. Before including the pull request please ensure:

* You include unit tests where appropriate (we are trying to improve the current unit testing).
* The project builds and the tests pass.

Don't worry about including a final build - we have a Jenkins server configured to automate the build so we can include that in the next release.

## C Sharp Source

The C# source includes a solution under C Sharp Source/LabVIEW CLI which consists of 3 projects.

* LabVIEW CLI is the main project.
* LabVIEW CLI WIX Setup contains the installer builder.
* LabVIEWCLI_Unit_tests contains unit testing for the project.

The solution is currently maintained in VS 2015.

Please include unit tests where appropriate for testing.

### Distributing

To update this distribution with changes you need to:

* Build the solution for "Any CPU" and "x64"
* Copy the 32 and 64 bit installers from the WIX project to Installation Support in the LabVIEW project using MoveInstallers.bat
* Redistribute the LabVIEW project.

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
