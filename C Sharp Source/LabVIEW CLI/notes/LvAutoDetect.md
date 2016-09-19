# LabVIEW versions auto-detection

Addresses Issue #1 on https://github.com/JamesMc86/LabVIEW-CLI

Inspired by Example-VI on NI Developer Community (https://decibel.ni.com/content/docs/DOC-25920)

## Concept

LabVIEW stores its Versions and Path info in subkeys of `HKEY_LOCAL_MACHINE\SOFTWARE\National Instruments\LabVIEW`.

The example-VI ignores the subkeys `AddOns` and `CurrentVersion`.

The subkeys have a value *VersionName* containing the Version String (i.e. `2012 SP1 f5` ) and *Path*
containing the Path to the directory which contains *LabVIEW.exe*.

## Use cases

### Nothing specified (default call)
If no executable is specified, the cli should use the Path value in the `CurrentVersion` key.

### Path to valid executable specified in --lv-exe
If there is a path to an executable specified, check if the file exists and ends with `.exe`. If it does, invoke it.

### --lv-exe specified, but no valid path
Try to resolve it to a discovered LabVIEW version (`2012` should resolve to `2012 SP1 f5`)


## 64bit
On an x64 operating system all 32bit programs store their keys below `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node`.

The software must check if it runs on a 64bit OS (`Environment.Is64BitOperatingSystem()`) and then try to find x64 versions of LabVIEW in the x64 hive.

The 32bit registry hive can be accessed reliably using `RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry32).OpenSubKey("SOFTWARE\\National Instruments\\LabVIEW");`
The 64bit registry hive can be accessed reliably using `RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry64).OpenSubKey("SOFTWARE\\National Instruments\\LabVIEW");`
 