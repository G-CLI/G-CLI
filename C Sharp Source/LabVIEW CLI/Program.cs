using System;
using System.Reflection;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;

namespace LabVIEW_CLI
{
    class Program
    {
        static bool connected = false;
        static bool stop = false;

        static int Main(string[] args)
        {
            int exitCode = 0;
            Output output = Output.Instance;

            string[] cliArgs, lvArgs;
            lvComms lvInterface = new lvComms();
            lvMsg latestMessage = new lvMsg("NOOP", "");
            LvLauncher launcher = null;
            CliOptions options = new CliOptions();

            lvVersion current = LvVersions.CurrentVersion;

            splitArguments(args, out cliArgs, out lvArgs);
            //CommandLine.Parser.Default.ParseArguments(cliArgs, options);
            if(!CommandLine.Parser.Default.ParseArguments(cliArgs, options))
            {
                Environment.Exit(CommandLine.Parser.DefaultExitCodeFail);
            }
            if (options.Version)
            {
                output.writeMessage(Assembly.GetExecutingAssembly().GetName().Version.ToString());
                Environment.Exit(0);
            }

            output.setVerbose(options.Verbose);
            output.writeInfo("LabVIEW CLI Started - Verbose Mode");
            output.writeInfo("Version " + Assembly.GetExecutingAssembly().GetName().Version);
            output.writeInfo("LabVIEW CLI Arguments: " + String.Join(" ", cliArgs));
            output.writeInfo("Arguments passed to LabVIEW: " + String.Join(" ", lvArgs));
            
            if (options.noLaunch)
            {
                output.writeMessage("Auto Launch Disabled");
                // disable timeout if noLaunch is specified
                options.timeout = -1;
            }
            else
            {
                // check launch vi
                if(options.LaunchVI == null)
                {
                    output.writeError("No launch VI supplied!");
                    return 1;
                }
                if (!File.Exists(options.LaunchVI))
                {
                    output.writeError("File \"" + options.LaunchVI + "\" does not exist!");
                    return 1;
                }

                List<string> permittedExtensions = new List<string>{ ".vi", ".lvproj" };
                string ext = Path.GetExtension(options.LaunchVI).ToLower();
                if (!permittedExtensions.Contains(ext))
                {
                    output.writeError("Cannot handle *" + ext + " files");
                    return 1;
                }

                try
                {
                    launcher = new LvLauncher(options.LaunchVI, lvPathFinder(options), lvInterface.port, lvArgs);
                    launcher.Exited += Launcher_Exited;
                    launcher.Start();
                }
                catch(KeyNotFoundException ex)
                {
                    // Fail gracefully if lv-ver option cannot be resolved
                    string bitness = options.x64 ? " 64bit" : string.Empty;
                    output.writeError("LabVIEW version \"" + options.lvVer + bitness + "\" not found!");
                    output.writeMessage("Available LabVIEW versions are:");
                    foreach(var ver in LvVersions.Versions)
                    {
                        output.writeMessage(ver.ToString());
                    }
                    return 1;
                }
                catch(FileNotFoundException ex)
                {
                    output.writeError(ex.Message);
                    return 1;
                }                    
            }

            // wait for the LabVIEW application to connect to the cli
            connected = lvInterface.waitOnConnection(options.timeout);

            // if timed out, kill LabVIEW and exit with error code
            if (!connected && launcher!=null)
            {
                output.writeError("Connection to LabVIEW timed out!");
                launcher.Kill();
                launcher.Exited -= Launcher_Exited;
                return 1;
            }

            do
            {
                latestMessage = lvInterface.readMessage();

                switch (latestMessage.messageType)
                {
                    case "OUTP":
                        Console.Write(latestMessage.messageData);
                        break;
                    case "EXIT":
                        exitCode = lvInterface.extractExitCode(latestMessage.messageData);
                        output.writeMessage("Recieved Exit Code " + exitCode);
                        stop = true;
                        break;
                    case "RDER":
                        exitCode = 1;
                        output.writeError("Read Error");
                        stop = true;
                        break;
                    default:
                        output.writeError("Unknown Message Type Recieved:" + latestMessage.messageType);
                        break;
                }


            } while (!stop);

            lvInterface.Close();
            return exitCode;
        }

        private static void Launcher_Exited(object sender, EventArgs e)
        {
            // Just quit by force if the tcp connection was not established or if LabVIEW exited without sending "EXIT" or "RDER"
            if (!connected || !stop)
            {
                Output output = Output.Instance;
                output.writeError("LabVIEW terminated unexpectedly!");
                Environment.Exit(1);
            }
        }

        private static void splitArguments(string[] args, out string[] cliArgs, out string[] lvArgs)
        {

            int splitterLocation = -1;

            for(int i = 0; i < args.Length; i++)
            {
                if(args[i] == "--")
                {
                    splitterLocation = i;
                }
            }

            if(splitterLocation > 0)
            {
                cliArgs = args.Take(splitterLocation).ToArray();
                lvArgs = args.Skip(splitterLocation + 1).ToArray();
            }
            else
            {
                cliArgs = args;
                lvArgs = new string[0];
            }

        }
        private static string lvPathFinder(CliOptions options)
        {
            if (options.lvExe != null)
            {
                if (!File.Exists(options.lvExe))
                    throw new FileNotFoundException("specified executable not found", options.lvExe);
                return options.lvExe;
            }
            if (options.lvVer != null)
            {
                try
                {
                    return LvVersions.ResolveVersionString(options.lvVer, options.x64).ExePath;
                }
                catch(KeyNotFoundException ex)
                {
                    throw; // So the exception makes it to the handler above.
                }
            }
            if (LvVersions.CurrentVersion != null)
            {
                return LvVersions.CurrentVersion.ExePath;
            }
            else
            {
                throw new FileNotFoundException("No LabVIEW.exe found...", "LabVIEW.exe");
            }
        }

    }


}
