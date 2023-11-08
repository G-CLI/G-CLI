# Tools That Work With G CLI

This page is intended to be a directory of the tools that have been created to work with G CLI, primarily for continuous integration.

## Shipping with G CLI

### Echo

Echos back any parameters you enter. Useful for testing the setup and install for a given LabVIEW version.

Example:
`g-cli --lv-ver 2011 echo -- one two three`

### Quit LabVIEW

Quits LabVIEW that is open. Maybe useful to control in a script when LabVIEW is closed.

Optional parameter -delay which specifies a time delay in ms. Default is 1000ms.

Example:
`g-cli --lv-ver 2011 quitLabVIEW -- -delay 500 `

### Clear Cache (from v3.0)

Clears the compiled object caches in LabVIEW.

Optionally pass `--user-only` for only the user compiled cache or `--app-builder-only` for just the app builder cache.

Example:
`g-cli --lv-ver 2011 ClearCache -- --user-only`

## From the G CLI Project

These tools are developed by the same people as G CLI and are all open source. Check the release section under code for the latest version.

* LabVIEW Builder - lvBuild - https://github.com/JamesMc86/G-CLI-lvBuild
* VI Package Manager Interface - https://github.com/JamesMc86/G-CLI-VI-Package-API
* VI Tester Runner - https://github.com/JamesMc86/G-CLI-VI-Tester-Runner

## Other open source projects powered by G CLI

* Caraya-CLI-extension - [Github](https://github.com/LabVIEW-Open-Source/Caraya-CLI-extension)
* CLI for Antidoc - [Gitlab](https://gitlab.com/wovalab/open-source/cli-for-antidoc) / [VIPM](https://www.vipm.io/package/wovalab_lib_antidoc_cli/)
* CLI for DQMH modules validation - [Gitlab](https://gitlab.com/dqmhconsortium/open-source/cli-module-validation) /  [VIPM](https://www.vipm.io/package/dqmh_consortium_lib_dqmh_cli_module_validation/)

## Commercial Tools

* Release Automation Tools from Hampel Software Engineering provides CI/CD tools that are powered by G CLI - https://rat.hampel-soft.com

## Legacy

The follow repos absolutely work, but they don't take advantage of the search integration within G CLI.

* LabVIEW CLI Common Steps. Github: https://github.com/LabVIEW-DCAF/buildsystem/.
* Command Line Tools for LabVIEW. Github: https://github.com/chinghwayu/Command-Line-Tools