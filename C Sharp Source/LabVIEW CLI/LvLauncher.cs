using System;
using Microsoft.Win32;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;

namespace LabVIEW_CLI
{
    class LvLauncher
    {
        private ProcessStartInfo procInfo;
        private Process process;
        private Output output = Output.Instance;

        public LvLauncher(String launchPath, String lvPath, int port, string[] args)
        {

            procInfo = new ProcessStartInfo();

            //Prepare the arguments with quotes.
            string preparedArgs = "";
            foreach (string arg in args)
            {
                preparedArgs += "\"" + arg + "\" ";
            }
            output.writeInfo("LabVIEW User Arguments: " + preparedArgs);

            string arguments = "-- -p:" + port + " " + preparedArgs;

            if (isExe(launchPath))
            {
                procInfo.FileName = launchPath;
                procInfo.Arguments = arguments;
            }
            else
            {
                procInfo.FileName = lvPath;
                procInfo.Arguments = "\"" + launchPath + "\" " + arguments;
            }
        }

        public void Start()
        {
            output.writeInfo("Launching Program:");
            output.writeInfo(procInfo.FileName + " " + procInfo.Arguments);
            process = Process.Start(procInfo);
        }

        private Boolean isExe(String launchPath)
        {
            return System.Text.RegularExpressions.Regex.IsMatch(launchPath, ".exe$");
        }

        /// <summary>
        /// Detects installed LabVIEW versions by searching the registry.
        /// 
        /// This follows an example from the NI Developer Community (https://decibel.ni.com/content/docs/DOC-25920).
        /// </summary>
        /// <returns>a dictionary containing the detected versions and the respective paths</returns>
        public static Dictionary<string, string> DetectLvVersions()
        {
            
            List<string> excludedKeys = new List<string> { "AddOns", "CurrentVersion" };
            Dictionary<string, string> versions = new Dictionary<string, string>();

            RegistryKey LvBaseKey = RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry32).OpenSubKey("SOFTWARE\\National Instruments\\LabVIEW");
            if (LvBaseKey == null)
            {
                Console.Error.WriteLine("No installed version of the NI LabVIEW Development System found...");
                return versions;
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
                    versions.Add(itemVersion.ToString(), itemPath.ToString());
                    Console.WriteLine("found \"" + itemVersion.ToString() + "\" (" + itemPath.ToString() + ")");
                }
            }

            return versions;
        }
    }
}
