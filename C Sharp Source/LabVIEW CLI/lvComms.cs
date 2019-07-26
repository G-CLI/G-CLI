using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Net;
using System.Net.Sockets;
using System.Net.NetworkInformation;

namespace G_CLI
{
    public class lvComms
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
        private const int MAX_PAYLOAD_BYTES = 1032 - LENGTH_BYTES - TYPE_BYTES; // 9000 is multiple of default MTU of 1500.

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

            //Assign a buffer for incoming data.
            _dataBuffer = new Byte[MAX_PAYLOAD_BYTES + TYPE_BYTES];

            //Start up the server
            _listener = new TcpListener(IPAddress.Parse("127.0.0.1"), _port);
            _listener.Start();
        }

        /// <summary>
        /// waits <paramref name="millisecondsTimeout"/> for a connection from the LabVIEW programm.
        /// </summary>
        /// <param name="millisecondsTimeout">The number of milliseconds to wait, or Infinite (-1) to wait indefinitely.</param>
        /// <returns>true if a connection was established, false if timed out</returns>
        public bool waitOnConnection(int millisecondsTimeout = -1) {

            //Blocking call to wait for TCP Client
            output.writeInfo("Waiting for connection on port " + _port);
            Task<TcpClient> clientTask = _listener.AcceptTcpClientAsync();
            if (!clientTask.Wait(millisecondsTimeout))
            {
                return false;
            }
            _client = clientTask.Result;
            _clientConnected = true;
            _stream = _client.GetStream();
            return true;
        }

        public int port {
            get { return _port;  }
        }

        public async Task<lvMsg> readMessage()
        {

            int bytesRead = 0, length = 0;
            Byte[] lengthBuff = new Byte[LENGTH_BYTES];

            try {
                bytesRead = await _stream.ReadAsync(lengthBuff, 0, LENGTH_BYTES);
            }
            catch(Exception ex)
            {
                return generateReadError("Read Length: Exception Found " + ex.ToString());
            }

            switch (bytesRead)
            {
                case LENGTH_BYTES:
                    Array.Reverse(lengthBuff);
                    length = BitConverter.ToInt32(lengthBuff, 0);
                    if(validLength(length))
                    {
                        return await readMessageData(length);
                    }
                    else
                    {
                        return generateReadError($"Bad Length Requested: {length}.");
                    }
                    break;
                case 0:
                    return generateReadError("No bytes at port. Client probably closed the connection prematurely");
                    break;
                default:
                    return generateReadError($"Only {bytesRead} bytes at port. Connection Error");
                    break;

            }

        }

        private async Task<lvMsg> readMessageData(int length)
        {

            int bytesRead;

            try
            {
                bytesRead = await _stream.ReadAsync(_dataBuffer, 0, length);
            }
            catch (Exception ex)
            {
                return generateReadError("Read Message: Exception Found " + ex.ToString());
            }

            switch(bytesRead)
            {
                case 0:
                    return generateReadError("0 bytes read from port. Connection was probably closed prematurely");
                default:
                    return decodeMessage(_dataBuffer, length);
            }


            
        }

        //Check length is greater than TYPE BYTES and less than the max payload.
        private bool validLength(int length)
        {
            return length >= TYPE_BYTES && length <= MAX_PAYLOAD_BYTES + TYPE_BYTES;
        }

        private lvMsg generateReadError(string errorMessage)
        {
            return new lvMsg("RDER", errorMessage);
        }

        private lvMsg decodeMessage(byte[] dataBuffer, int length)
        {
            string msgType = "", msgData = "";

            msgType = Encoding.ASCII.GetString(dataBuffer, 0, TYPE_BYTES);

            //If we have message data to read.
            if (length > TYPE_BYTES)
            {
                msgData = Encoding.ASCII.GetString(_dataBuffer, TYPE_BYTES, length - TYPE_BYTES);
            }
            else
            {
                msgData = "";
            }

            return new lvMsg(msgType, msgData);
        }

        //Writes the messages required to start the CLI interface.
        public void writeInitialMessages(string[] lvArgs, string currentDirectory)
        {
            writeMessage(generateArgumentsMessage(lvArgs));
            writeMessage(new lvMsg("CCWD", currentDirectory));
        }

        private void writeMessage(lvMsg messageToWrite)
        {

            byte[] buffer = new byte[MAX_PAYLOAD_BYTES];
            int messageDataSize = messageToWrite.messageData.Length;
            byte[] binaryLength = new byte[4];

            if (messageDataSize > 1024)
            {
                throw new ArgumentOutOfRangeException("Message Data Length", messageDataSize, "Must be 1024 bytes max");
            }

            //First sort the overall length
            binaryLength = BitConverter.GetBytes(messageDataSize + 4);
            Array.Reverse(binaryLength); //To big endian
            Array.Copy(binaryLength, buffer, 4);

            //Now write message type and data.
            Array.Copy(Encoding.ASCII.GetBytes(messageToWrite.messageType), 0, buffer, 4, 4);
            Array.Copy(Encoding.ASCII.GetBytes(messageToWrite.messageData), 0, buffer, 8, messageDataSize);

            _stream.Write(buffer, 0, messageDataSize + 8);
        }

        public int extractExitCode(string msgData)
        {

            return Int32.Parse(msgData);

        }

        public lvMsg generateArgumentsMessage(string[] lvArgs)
        {
            return new lvMsg("ARGS", String.Join("\t", lvArgs));
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

    public class lvMsg
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
