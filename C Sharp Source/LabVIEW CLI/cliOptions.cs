using System;
using System.Text;
using CommandLine;
using System.Collections.Generic;
using CommandLine.Text;

namespace LabVIEW_CLI
{
    public class CliOptions
    {
        [Option("version", DefaultValue = false,
        HelpText = "Prints the program version.")]
        public bool Version { get; set; }

        [Option('v', "verbose", DefaultValue = false,
        HelpText = "Prints all messages to standard output.")]
        public bool Verbose { get; set; }

        [Option("no-launch", DefaultValue = false,
        HelpText = "Prevents launching the LabVIEW app but waits for connection instead. Overrides --timeout to -1.")]
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

        [Option("timeout", DefaultValue = 60000,
        HelpText = "Maximum time (in ms) to wait for the LabVIEW program to connect to the cli. -1 = forever. Default is 60000 (60 seconds)")]
        public int timeout { get; set; }

        [Option("kill", DefaultValue = false,
        HelpText = "Forces the LabVIEW process to exit after the CLI receives a return code/error")]
        public bool kill { get; set; }

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
