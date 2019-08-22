using System;
using Microsoft.Win32;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Threading;
using System.IO;

namespace G_CLI
{
    public class LvLauncher
    {
        
        private Process lvProcess;
        private Output output = Output.Instance;
        private Thread LvTrackingThread;
        private ManualResetEventSlim lvStarted = new ManualResetEventSlim();
        private Boolean cliExited = false;
        private Boolean processSwitched = false;

        public ProcessStartInfo procInfo { get; }
        public ManualResetEventSlim lvExited = new ManualResetEventSlim();


        public event EventHandler Exited;

        public LvLauncher(string launchPath, lvVersion lvVer, int port, portRegistration portRegistration)
        {

            procInfo = new ProcessStartInfo();

            string arguments = "-unattended -- -p:" + port;

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


                if(!File.Exists(launchPath))
                {
                    //The file doesn't exist. Check search directory.
                    launchPath = substituteSearchPath(launchPath, lvVer);

                    if(!File.Exists(launchPath))
                    {
                        //Still doesn't exists - throw exception.
                        throw new FileNotFoundException("Launch path does not exist locally or in the the tools directory", launchPath);
                    }
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
            if (lvProcess.HasExited == false)
            {
                output.writeInfo("killing LabVIEW process (" + lvProcess.Id + ")");
                lvProcess.Kill();
            }
        }

        //Closes the tracking thread to ensure we don't leave it active.
        public void Close()
        {
            cliExited = true;
            //block this call until the thread has stopped.
            LvTrackingThread.Join();
        }

        /// <summary>
        /// Starts the LabVIEW process and keeps track of it.
        /// Must be run in a dedicated thread.
        /// </summary>
        private void _processTrackingThread()
        {
            System.Threading.Timer refreshTimer;
            AutoResetEvent endEvent = new AutoResetEvent(false);

            lvProcess.EnableRaisingEvents = true;
            lvProcess.Start();
            output.writeInfo("LabVIEW/App started, process ID is " + lvProcess.Id.ToString());
            lvStarted.Set();

            lvProcess.Exited += Process_Exited;

            refreshTimer = new System.Threading.Timer(refreshProcessState, endEvent, 0, 500);

            //wait for end event from process state check.
            endEvent.WaitOne();
            refreshTimer.Dispose();
            output.writeInfo("Process Tracking Thread Finished.");
        }

        //Callback method for the timer. Sets the AutoResetEvent when the process should stop.
        private void refreshProcessState(Object stateInfo) {

            AutoResetEvent endEvent = (AutoResetEvent)stateInfo;

            lvProcess.Refresh();

            if(lvExited.IsSet || cliExited)
            {
                endEvent.Set();
            }
        }

        private Boolean isExe(String launchPath)
        {
            return System.Text.RegularExpressions.Regex.IsMatch(launchPath, ".exe$", System.Text.RegularExpressions.RegexOptions.IgnoreCase);
        }

        private void Process_Exited(object sender, EventArgs e)
        {
            output.writeInfo("LabVIEW process exited. Checking for process switch");

            Process newProcess = findLabVIEWProcessByPath(lvProcess.StartInfo.FileName);
            if (newProcess != null) //found a process.
            {
                output.writeInfo("LabVIEW still found with PID " + newProcess.Id.ToString());
                lvProcess = newProcess;
                lvProcess.EnableRaisingEvents = true;
                lvProcess.Exited += Process_Exited; //re-attach this event call to the new process.
                processSwitched = true;
            }
            else
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


        }

        private Process findLabVIEWProcessByPath(string path)
        {
            Process[] AllMatching;
            Process FullMatch = null;

            AllMatching = Process.GetProcessesByName("LabVIEW");

            foreach(Process currentProcess in AllMatching)
            {
                try
                {
                    string modulePath = currentProcess.MainModule.FileName;
                    if (modulePath == path)
                    {
                        FullMatch = currentProcess;
                        return FullMatch;
                    }
                }
                catch(System.ComponentModel.Win32Exception e)
                {
                    //Seen this exception in some cases. Catch and report so we can understand when it occurs. writeInf could be removed in future.
                    output.writeDebug("Exception accessing " + currentProcess.ProcessName + " pid " + currentProcess.Id);
                    output.writeDebug(e.Message);
                }
            }

            return FullMatch;
            

        }

        //This function will complete the launch path to a file in the search directory if it doesn't
        //exist locally. If it exists locally it will do nothing.
        private string substituteSearchPath(string launchPath, lvVersion labview) {

            string searchPath;
        
            searchPath = Path.Combine(labview.ToolsPath, launchPath);
            output.writeDebug(String.Format("Looks like the requested VI {0} Doesnt Exist. Checking In Search Path Instead: {1}", launchPath, searchPath));
            return searchPath;

        }

    }
}
