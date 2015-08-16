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
        private String lvPath = "C:\\Program Files (x86)\\National Instruments\\LabVIEW 2014\\labview.exe";
        private ProcessStartInfo procInfo;
        private Process process;

        public LvLauncher(String launchPath)
        {

            procInfo = new ProcessStartInfo();

            if(isExe(launchPath))
            {
                procInfo.FileName = launchPath;
            }
            else
            {
                procInfo.FileName = lvPath;
                procInfo.Arguments = "\"" + launchPath + "\"";
            }
        }

        public void Start()
        {
            process = Process.Start(procInfo);
        }

        private Boolean isExe(String launchPath)
        {
            return System.Text.RegularExpressions.Regex.IsMatch(launchPath, ".exe$");
        }
    }
}
