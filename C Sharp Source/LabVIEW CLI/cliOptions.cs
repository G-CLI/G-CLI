using System;
using System.Text;
using CommandLine;


namespace LabVIEW_CLI
{
    public class CliOptions


    {

        [Option('v', "verbose", DefaultValue = false,
        HelpText = "Prints all messages to standard output.")]
        public bool Verbose { get; set; }

        [Option("no-launch", DefaultValue = false,
        HelpText = "Prevents launching the LabVIEW App But Waits for Connection Instead")]
        public bool noLaunch { get; set; }

        [Option("lv-exe", DefaultValue = null,
        HelpText = "Sets the LabVIEW Executable to launch VIs with")]
        public string lvExe { get; set; }

        [HelpOption('h',"help", HelpText = "Dispaly this help screen.")]
        public string GetUsage()
        {
            return "Help Screen" + Environment.NewLine;
        }
    }
}
