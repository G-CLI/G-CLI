using System;
using Microsoft.Win32;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Threading;

namespace LabVIEW_CLI
{
    class LvLauncher
    {
        private ProcessStartInfo procInfo;
        private Process process;
        private Output output = Output.Instance;
        private Thread LvTrackingThread;

        public event EventHandler Exited;

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

            process = new Process();
            process.StartInfo = procInfo;
            process.Exited += Process_Exited;
        }

        public void Start()
        {
            output.writeInfo("Launching Program:");
            output.writeInfo(procInfo.FileName + " " + procInfo.Arguments);
            LvTrackingThread = new Thread(new ThreadStart(_processTrackingThread)) { Name = "LvTrackingThread" };
            LvTrackingThread.Start();
        }

        /// <summary>
        /// Starts the LabVIEW process and keeps track of it.
        /// Must be run in a dedicated threat.
        /// </summary>
        private void _processTrackingThread()
        {
            process.Start();
            output.writeInfo("LabVIEW started, process ID is " + process.Id.ToString());

            while (!process.HasExited)
            {
                process.Refresh();
                Thread.Sleep(500);
            }

            output.writeInfo("process tracking tread finished");
        }

        private Boolean isExe(String launchPath)
        {
            return System.Text.RegularExpressions.Regex.IsMatch(launchPath, ".exe$");
        }

        private void Process_Exited(object sender, EventArgs e)
        {
            output.writeInfo("LabVIEW exiting...");
            Exited?.Invoke(sender, e);
        }

    }
}
