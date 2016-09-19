using System;
using System.Collections.Generic;
using System.Linq;
using Microsoft.Win32;
using System.Text;
using System.Threading.Tasks;

namespace LabVIEW_CLI
{
    class LvVersions
    {
        public List<lvVersion> Versions { get; private set; }

        /// <summary>
        /// Detects installed LabVIEW versions by searching the registry.
        /// 
        /// This follows an example from the NI Developer Community (https://decibel.ni.com/content/docs/DOC-25920).
        /// See also in notes/LvAutoDetect.md
        /// </summary>
        public LvVersions()
        {
            List<string> excludedKeys = new List<string> { "AddOns", "CurrentVersion" };
            Versions = new List<lvVersion>();

            RegistryKey LvBaseKey = RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry32).OpenSubKey("SOFTWARE\\National Instruments\\LabVIEW");
            if (LvBaseKey == null)
            {
                Console.Error.WriteLine("No installed version of the NI LabVIEW Development System found...");
                return;
            }

            RegistryKey itemKey;
            object itemPath;
            object itemVersion;
            foreach (var item in LvBaseKey.GetSubKeyNames())
            {
                Console.WriteLine("Testing key \"" + item + "\"...");
                if (excludedKeys.Contains(item))
                    continue;

                itemKey = LvBaseKey.OpenSubKey(item);
                itemPath = itemKey.GetValue("Path");
                itemVersion = itemKey.GetValue("VersionString");
                if (itemPath != null && itemVersion != null)
                {
                    Versions.Add(new lvVersion{ Version = itemVersion.ToString(), Path = itemPath.ToString(), Architecture = "32bit" });
                    Console.WriteLine("found \"" + itemVersion.ToString() + "\" (" + itemPath.ToString() + ")");
                }
            }
        }
    }


    class lvVersion
    {
        public string Version { get; set; }

        public string Path { get; set; }
            
        public string Architecture { get; set; }
    }
}
