using System;
using System.Reflection;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LabVIEW_CLI
{
    class Program
    {
        static int Main(string[] args)
        {

            Boolean stop = false;
            int exitCode = 0;
            Output output = Output.Instance;

            string[] cliArgs, lvArgs;
            lvComms lvInterface = new lvComms();
            lvMsg latestMessage = new lvMsg("NOOP", "");
            LvLauncher launcher;
            CliOptions options = new CliOptions();
            Dictionary<string,string> versions = LvLauncher.DetectLvVersions();

            if(versions.Count > 0)
            {
                output.writeInfo("Detected the following versions of LabVIEW:");
                foreach (var item in versions)
                {
                    output.writeInfo(item.Key + ": " + item.Value);
                }
            }
                

            splitArguments(args, out cliArgs, out lvArgs);
            CommandLine.Parser.Default.ParseArguments(cliArgs, options);

            if (cliArgs.Length < 1)
            {
                output.writeError("No Arguments Used");
                return -1;
            }
            else
            {
                string launchPath = cliArgs[cliArgs.Length - 1];

                output.setVerbose(options.Verbose);
                output.writeInfo("LabVIEW CLI Started - Verbose Mode");
                output.writeInfo("Version " + Assembly.GetExecutingAssembly().GetName().Version);
                output.writeInfo("LabVIEW CLI Arguments: " + String.Join(" ", cliArgs));


                // Args don't include the exe name.
                if (options.noLaunch)
                {
                    output.writeMessage("Auto Launch Disabled");
                }
                else
                {
                    launcher = new LvLauncher(launchPath, lvPathFinder(options), lvInterface.port, lvArgs);
                    launcher.Start();
                }

                lvInterface.waitOnConnection();

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
            if (options.lvExe == null)
            {
                return "C:\\Program Files (x86)\\National Instruments\\LabVIEW 2014\\labview.exe";
            }
            else
            {
                return options.lvExe;
            }
        }

    }


}
