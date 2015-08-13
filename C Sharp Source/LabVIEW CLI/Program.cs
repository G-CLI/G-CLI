using System;
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
            lvComms lvInterface = new lvComms();
            lvMsg latestMessage = new lvMsg("NOOP", "");



            // Args don't include the exe name.

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
                        Console.WriteLine("Recieved Exit Code " + exitCode);
                        stop = true;
                        break;
                    default:
                        Console.WriteLine("Unknown Message Type Recieved:" + latestMessage.messageType);
                        break;
                }


            } while (!stop);

            lvInterface.Close();
            return exitCode;
        }
    }
}
