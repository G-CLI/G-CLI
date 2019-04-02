using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LabVIEW_CLI
{
    class Output
    {
        private static Output instance;
        private bool verbose;
        private bool quiet;

        private Output() { }

        public static Output Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new Output();
                }
                return instance;
            }
        }

        public void setVerbose(bool verboseIn)
        {
            verbose = verboseIn;
        }
        
        public void setQuiet(bool quietIn)
        {
            quiet = quietIn;
        }

        public void writeError(string message)
        {
            Console.Error.WriteLine(message);
        }

        public void writeMessage(string message)
        {
            if (!quiet)
            {
                Console.WriteLine(message);
            }
        }

        public void writeInfo(string message)
        {
            if (verbose)
            {
                Console.WriteLine(message);
            }
        }
    }
}
