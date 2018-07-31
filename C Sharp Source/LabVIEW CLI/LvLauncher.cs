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
        private Process lvProcess;
        private Output output = Output.Instance;
        private Thread LvTrackingThread;
        private ManualResetEventSlim lvStarted = new ManualResetEventSlim();
        private Boolean cliExited = false;
        private Boolean processSwitched = false;

        public int ProcessId { get; private set; }
        public ManualResetEventSlim lvExited = new ManualResetEventSlim();


        public event EventHandler Exited;

        public LvLauncher(string launchPath, lvVersion lvVer, int port, portRegistration portRegistration)
        {

            procInfo = new ProcessStartInfo();

            string arguments = "-- -p:" + port;

            if (isExe(launchPath))
            {
                procInfo.FileName = launchPath;
                procInfo.Arguments = arguments;
            }
            else
            {
                //gate on a blank LV path which means we don't have LabVIEW Installed.
                if(lvVer.ExePath == "")
                {
                    throw new System.IO.FileNotFoundException("No LabVIEW.exe found...", "LabVIEW.exe");
                }
                procInfo.FileName = lvVer.ExePath;
                procInfo.Arguments = "\"" + launchPath + "\" " + arguments;
                portRegistration.registerPort(launchPath, lvVer, port);
            }

            lvProcess = new Process();
            lvProcess.StartInfo = procInfo;
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
            output.writeInfo("Process Name " + lvProcess.ProcessName.ToString());
            lvStarted.Set();

            //wait for once second to work out if the process has held the PID or connected to existing process.
            Thread.Sleep(500);
            lvProcess.Refresh();

            //If it has exited we will attempt to recover a different PID.
            if(lvProcess.HasExited)
            {
                output.writeInfo("LabVIEW process exited rapidly. Checking for process switch");

                Process newProcess = findLabVIEWProcessByPath(lvProcess.StartInfo.FileName);
                if(newProcess != null) //found a process.
                {
                    output.writeInfo("LabVIEW still found with PID " + newProcess.Id.ToString());
                    lvProcess = newProcess;
                    processSwitched = true;
                }     
                else
                {
                    //call process exited.
                    Process_Exited(lvProcess, null); //fake event.
                }     
            }


            lvProcess.Exited += Process_Exited;

            while (!lvProcess.HasExited && !cliExited)
            {
                lvProcess.Refresh();
                Thread.Sleep(500);
            }

            output.writeInfo("Process Tracking Thread Finished.");
        }

        private Boolean isExe(String launchPath)
        {
            return System.Text.RegularExpressions.Regex.IsMatch(launchPath, ".exe$", System.Text.RegularExpressions.RegexOptions.IgnoreCase);
        }

        private void Process_Exited(object sender, EventArgs e)
        {
            output.writeInfo("LabVIEW/App exiting...");

            //check exit code if we havent switched processes. (If we have then this is unsupported.)
            if (!processSwitched)
            {
                //notify if it looks like it wasn't a clean exit.
                if (lvProcess.ExitCode != 0)
                {
                    output.writeError("LabVIEW exited with code " + lvProcess.ExitCode.ToString());
                }
                else
                {
                    output.writeInfo("LabVIEW exited with code " + lvProcess.ExitCode.ToString());
                }
            }

            lvExited.Set();
            Exited?.Invoke(sender, e);
        }

        private Process findLabVIEWProcessByPath(string path)
        {
            Process[] AllMatching;
            Process FullMatch = null;

            AllMatching = Process.GetProcessesByName("LabVIEW");

            foreach(Process currentProcess in AllMatching)
            {
                string modulePath = currentProcess.MainModule.FileName;
                if(modulePath == path)
                {
                    FullMatch = currentProcess;
                    return FullMatch;
                }
            }

            if(FullMatch == null)
            {
                throw new System.Exception("Not Found");
            }

            return FullMatch;
            

        }

    }
}
