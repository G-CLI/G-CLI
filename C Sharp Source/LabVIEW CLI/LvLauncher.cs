﻿using System;
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
        private Process lvProcess;
        private Output output = Output.Instance;
        private Thread LvTrackingThread;
        private ManualResetEventSlim lvStarted = new ManualResetEventSlim();
        private Boolean cliExited = false;

        public int ProcessId { get; private set; }
        public ManualResetEventSlim lvExited = new ManualResetEventSlim();


        public event EventHandler Exited;

        public LvLauncher(string launchPath, string lvPath, int port, string[] args)
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
                if (isLabVIEWRunning(lvPath))
                {
                    output.writeMessage("It looks like LabVIEW is already running. The launch may fail.");
                }

                    procInfo.FileName = lvPath;
                procInfo.Arguments = "\"" + launchPath + "\" " + arguments;
            }

            lvProcess = new Process();
            lvProcess.StartInfo = procInfo;
            lvProcess.Exited += Process_Exited;
        }

        public void Start()
        {
            output.writeInfo("Launching Program:");
            output.writeInfo(procInfo.FileName + " " + procInfo.Arguments);
            LvTrackingThread = new Thread(new ThreadStart(_processTrackingThread)) { Name = "LvTrackingThread" };
            LvTrackingThread.Start();
            lvStarted.Wait();
        }

        public void Kill()
        {
            output.writeInfo("killing LabVIEW process (" + ProcessId + ")");
            lvProcess.Kill();
        }

        //Closes the tracking thread to ensure we don't leave it active.
        public void Close()
        {
            cliExited = true;
        }

        /// <summary>
        /// Starts the LabVIEW process and keeps track of it.
        /// Must be run in a dedicated thread.
        /// </summary>
        private void _processTrackingThread()
        {
            lvProcess.Start();
            ProcessId = lvProcess.Id;
            output.writeInfo("LabVIEW/App started, process ID is " + lvProcess.Id.ToString());
            lvStarted.Set();

            while (!lvProcess.HasExited && !cliExited)
            {
                lvProcess.Refresh();
                Thread.Sleep(500);
            }

            output.writeInfo("process tracking thread finished");
        }

        private Boolean isExe(String launchPath)
        {
            return System.Text.RegularExpressions.Regex.IsMatch(launchPath, ".exe$");
        }

        private Boolean isLabVIEWRunning(String path)
        {

            Process[] lvProcesses = Process.GetProcessesByName("LabVIEW");
            Process matchingProcess = lvProcesses.FirstOrDefault(p => p.MainModule.FileName.Equals(path));
            bool isRunning = matchingProcess != default(Process);

            return isRunning;
        }

        private void Process_Exited(object sender, EventArgs e)
        {
            output.writeInfo("LabVIEW/App exiting...");

            //notify if it looks like it wasn't a clean exit.
            if(lvProcess.ExitCode != 0)
            {
                output.writeError("LabVIEW exited with code " + lvProcess.ExitCode.ToString());
            }
            else
            {
                output.writeInfo("LabVIEW exited with code " + lvProcess.ExitCode.ToString());
            }


            lvExited.Set();
            Exited?.Invoke(sender, e);
        }

    }
}
