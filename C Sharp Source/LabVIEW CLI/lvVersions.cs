using System;
using System.Collections.Generic;
using Microsoft.Win32;
using System.Linq;

namespace LabVIEW_CLI
{
    static class LvVersions
    {
        private const string BASE_KEY = "SOFTWARE\\National Instruments\\LabVIEW";
        private static readonly List<string> excludedKeys = new List<string> { "AddOns", "CurrentVersion" };

        public static List<lvVersion> Versions { get; private set; }
        public static lvVersion CurrentVersion { get; private set; }

        /// <summary>
        /// Detects installed LabVIEW versions by searching the registry.
        /// 
        /// This follows an example from the NI Developer Community (https://decibel.ni.com/content/docs/DOC-25920).
        /// See also in notes/LvAutoDetect.md
        /// </summary>
        static LvVersions()
        {
            Scan();
        }

        /// <summary>
        /// Triggers a registry scan to determine the currently installed LabVIEW versions.
        /// This method is automatically invoked by the static constructor.
        /// </summary>
        public static void Scan()
        {
            // Initialize Versions List
            Versions = new List<lvVersion>();
            CurrentVersion = null;

            // Scan 32bit Registry
            _ScanRegistry(RegistryView.Registry32);

            // Scan 64bit Registry on x64 Systems
            if (Environment.Is64BitOperatingSystem)
                _ScanRegistry(RegistryView.Registry64);

            // Determine Current Version (default if no exe or version is supplied as argument)
            _GetCurrentVersion();

            Output output = Output.Instance;
            if (CurrentVersion == null)
            {
                output.writeInfo("No installed LabVIEW version discovered!");
            }
            else
            {
                output.writeInfo("Current Version: " + CurrentVersion.ToString());

                output.writeInfo("Detected LabVIEW versions:");
                foreach(var current in Versions)
                {
                    output.writeInfo(current.ToString() + "(" + current.Path + ")");
                }
            }
            
        }

        /// <summary>
        /// Tries to resolve the supplied string to one of the discovered LabVIEW versions
        /// </summary>
        /// <param name="versionString">the version string from the command line</param>
        /// <exception cref="KeyNotFoundException">thrown if the versionString cannot be resolved</exception>
        /// <returns></returns>
        public static lvVersion ResolveVersionString(string versionString, bool x64 = false)
        {
            var results = from ver in Versions where ver.Version.Contains(versionString) orderby ver.Bitness ascending select ver;
            if (results.Count() <= 0)
                throw new KeyNotFoundException();

            // if 64-bit is not requested, return the first entry (which might be x64)
            if(!x64)
                return results.First();

            // else filter out all non-x64 versions
            var x64results = from ver in results where ver.Bitness == "64bit" select ver;
            if (x64results.Count() > 0)
                return x64results.First();

            throw new KeyNotFoundException();
        }
        

        /// <summary>
        /// Scans the registry for installed LabVIEW versions
        /// </summary>
        /// <param name="regView">32bit or 64bit registry</param>
        private static void _ScanRegistry(RegistryView regView)
        {
            string bitness = (regView == RegistryView.Registry32) ? "32bit" : "64bit";

            RegistryKey baseKey = _GetBaseKey(regView);
            if (baseKey == null)
                return;


            RegistryKey itemKey;
            object itemPath;
            object itemVersion;
            object itemGUID;
            foreach (var item in baseKey.GetSubKeyNames())
            {
                if (excludedKeys.Contains(item))
                    continue;

                itemKey = baseKey.OpenSubKey(item);
                itemPath = itemKey.GetValue("Path");
                itemVersion = itemKey.GetValue("VersionString");
                itemGUID = itemKey.GetValue("GUID");
                if (itemPath != null && itemVersion != null)
                {
                    Versions.Add(new lvVersion { Version = itemVersion.ToString(), Path = itemPath.ToString(), Bitness = bitness, GUID = itemGUID.ToString() });
                }
            }
        }
        
        /// <summary>
        /// Determines the default LabVIEW version.
        /// Favours 32bit over 64bit version, if both coexist
        /// Falls back to using the first entry of the versions list,
        /// </summary>
        private static void _GetCurrentVersion()
        {
            // don't proceed if the scans did not discover any lv versions
            if (Versions.Count <= 0)
                return;

            // check 32bit key
            _CheckCurrentVersionKey(RegistryView.Registry32);
            if (CurrentVersion != null)
                return; //exit if found

            // check 64bit
            if (Environment.Is64BitOperatingSystem)
                _CheckCurrentVersionKey(RegistryView.Registry64);
            if (CurrentVersion != null)
                return; //exit if found

            // use first entry in Versions
            CurrentVersion = Versions.First();
        }

        private static void _CheckCurrentVersionKey(RegistryView view)
        {
            RegistryKey currentKey = _GetBaseKey(view).OpenSubKey("CurrentVersion");
            if (currentKey != null)
            {
                object GUID = currentKey.GetValue("GUID");
                if (GUID != null)
                {
                    foreach (lvVersion current in Versions)
                    {
                        if (current.GUID == GUID.ToString())
                        {
                            CurrentVersion = current;
                            break;
                        }
                    }
                }
            }
        }

        /// <summary>
        /// Gets the base registry key for NI LabVIEW installations
        /// </summary>
        /// <param name="view">Use 32-bit or 64-bit registry</param>
        /// <returns>the base registry key</returns>
        private static RegistryKey _GetBaseKey(RegistryView view)
        {
            return RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, view).OpenSubKey(BASE_KEY);
        }
    }


    class lvVersion
    {
        public string Version { get; set; }

        public string Path { get; set; }
            
        public string Bitness { get; set; }

        public string GUID { get; set; }

        public string ExePath
        {
            get
            {
                return System.IO.Path.Combine(Path, "LabVIEW.exe");
            }
        }

        public override string ToString()
        {
            return Version + ", " + Bitness; 
        }
    }
}
