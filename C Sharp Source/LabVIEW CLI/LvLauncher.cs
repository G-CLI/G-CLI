using System;
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

            string arguments = "-- -p:" + port + " " + String.Join(" ", args);

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
    }
}
