using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Net;
using System.Net.Sockets;
using System.Net.NetworkInformation;

namespace LabVIEW_CLI
{
    class lvComms
    {
        private Output output = Output.Instance;
        private TcpListener _listener;
        private TcpClient _client;
        private NetworkStream _stream;
        private int _port;
        private Boolean _clientConnected = false;
        private Byte[] _dataBuffer;
        private const int LENGTH_BYTES = 4;
        private const int TYPE_BYTES = 4;
        private const int MAX_PAYLOAD_BYTES = 9000 - LENGTH_BYTES - TYPE_BYTES; // 9000 is multiple of default MTU of 1500.

        // Returns available port number or zero if no port is available
        public static int GetFirstAvailableRandomPort(int startPort, int stopPort)
        {
            Random r = new Random();

            IPGlobalProperties ipGlobalProperties = IPGlobalProperties.GetIPGlobalProperties();
            TcpConnectionInformation[] tcpConnInfoArray = ipGlobalProperties.GetActiveTcpConnections();

            var busyPorts = tcpConnInfoArray.Select(t => t.LocalEndPoint.Port).Where(v => v >= startPort && v <= stopPort).ToArray();

            var firstAvailableRandomPort = Enumerable.Range(startPort, stopPort - startPort).OrderBy(v => r.Next()).FirstOrDefault(p => !busyPorts.Contains(p));
        
            return firstAvailableRandomPort;
        }

        public lvComms()
        {
            //Get first available dynamic port
            _port = GetFirstAvailableRandomPort(49152, 65535);

            //Assign a buffer for incoming data. 1k should be plenty.
            _dataBuffer = new Byte[MAX_PAYLOAD_BYTES + TYPE_BYTES];

            //Start up the server
            _listener = new TcpListener(IPAddress.Parse("127.0.0.1"), _port);
            _listener.Start();
        }

        public void waitOnConnection() {

            //Blocking call to wait for TCP Client
            output.writeInfo("Waiting for connection on port " + _port);
            _client = _listener.AcceptTcpClient();
            _clientConnected = true;
            _stream = _client.GetStream();
            output.writeInfo("Client Connected");
        }

        public int port {
            get { return _port;  }
        }

        public lvMsg readMessage()
        {

            int bytesRead = 0, length = 0;
            Byte[] lengthBuff = new Byte[LENGTH_BYTES];
            string msgType = "", msgData = "";

            try {
                bytesRead = _stream.Read(lengthBuff, 0, LENGTH_BYTES);
            }
            catch(Exception ex)
            {
                Console.WriteLine("Exception Found");
            }
            if (bytesRead == LENGTH_BYTES)
            {
                Array.Reverse(lengthBuff);
                length = BitConverter.ToInt32(lengthBuff, 0);
                if (length > TYPE_BYTES)
                {
                    _stream.Read(_dataBuffer, 0, length);
                    msgType = Encoding.ASCII.GetString(_dataBuffer, 0, TYPE_BYTES);
                    msgData = Encoding.ASCII.GetString(_dataBuffer, TYPE_BYTES, length - TYPE_BYTES);
                }

                return new lvMsg(msgType, msgData);
            }
            else
            {
                return new lvMsg("RDER", "");
            }
        }

        public int extractExitCode(string msgData)
        {
            Byte[] codeBytes = new Byte[4];

            codeBytes = Encoding.ASCII.GetBytes(msgData.ToCharArray(0, 4), 0, 4);
            Array.Reverse(codeBytes);

            return BitConverter.ToInt32(codeBytes, 0);

        }

        public void Close()
        {
            if(_clientConnected)
            {
                _client.Close();
            }
            _listener.Stop();
        }
    }

    class lvMsg
    {
        private string _messageType;
        private string _messageData;

        public lvMsg(string messageType, string messageData)
        {
            this._messageType = messageType;
            this._messageData = messageData;
        }

        public string messageType
        {
            get { return _messageType; }
        }

        public string messageData
        {
            get { return _messageData; }
        }
    }
}
