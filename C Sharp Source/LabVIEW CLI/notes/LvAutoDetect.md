# LabVIEW versions auto-detection

Addresses Issue #1 on https://github.com/JamesMc86/LabVIEW-CLI

Inspired by Example-VI on NI Developer Community (https://decibel.ni.com/content/docs/DOC-25920)

## Concept

LabVIEW stores its Versions and Path info in subkeys of `HKEY_LOCAL_MACHINE\SOFTWARE\National Instruments\LabVIEW`.

The example-VI ignores the subkeys `AddOns` and `CurrentVersion`.

The subkeys have a value *VersionName* containing the Version String (i.e. `2012 SP1 f5` ) and *Path*
containing the Path to the directory which contains *LabVIEW.exe*.

## 64bit
On an x64 operating system all 32bit programs store their keys below `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node`.

Therefore the software must check if it runs on a 64bit OS (`Environment.Is64BitOperatingSystem()`)

The 32bit registry hive can be accessed using `RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry32).OpenSubKey("SOFTWARE\\National Instruments\\LabVIEW");`
The 64bit registry hive can be accessed using `RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry64).OpenSubKey("SOFTWARE\\National Instruments\\LabVIEW");`
 