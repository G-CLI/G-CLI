using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Net;
using System.Net.Sockets;

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

        public lvComms()
        {
            _port = 5001;

            //Assign a buffer for incoming data. 1k should be plenty.
            _dataBuffer = new Byte[1024];

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
            int length = 0;
            Byte[] lengthBuff = new Byte[4];
            string msgType = "", msgData = "";

            _stream.Read(lengthBuff, 0, 4);
            Array.Reverse(lengthBuff);
            length = BitConverter.ToInt32(lengthBuff, 0);
            if (length > 4)
            {
                _stream.Read(_dataBuffer, 0, length);
                msgType = Encoding.ASCII.GetString(_dataBuffer, 0, 4);
                msgData = Encoding.ASCII.GetString(_dataBuffer, 4, length - 4);
            }

            return new lvMsg(msgType, msgData);
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
