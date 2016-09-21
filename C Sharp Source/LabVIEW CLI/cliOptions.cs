using System;
using System.Text;
using CommandLine;
using System.Collections.Generic;
using CommandLine.Text;

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

        [Option("lv-ver", DefaultValue = null,
        HelpText = "Use the specified LabVIEW version from auto-discover")]
        public string lvVer { get; set; }

        [Option("x64", DefaultValue = false,
        HelpText = "Use a 64-bit LabVIEW exe")]
        public bool x64 { get; set; }

        [ValueOption(0)]
        public string LaunchVI { get; set; }

        [HelpOption('h',"help", HelpText = "Display this help screen.")]
        public string GetUsage()
        {
            return HelpText.AutoBuild(this,
                (HelpText current) => HelpText.DefaultParsingErrorsHandler(this, current));
        }
    }
}
