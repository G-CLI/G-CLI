using System;
using System.IO;
using System.Net.Http;
using System.Text.RegularExpressions;

namespace G_CLI
{
    public class portRegistration
    {
        private Boolean _registered;
        private HttpClient _httpClient;
        private string _launchID;

        public portRegistration()
        {
            _httpClient = new HttpClient();
            _registered = false;

        }

        public void registerPort(string viPath, lvVersion lvVer, int port)
        {
            CreateLaunchID(viPath, lvVer);

            string baseResponse = "=HTTP/1.0 200 OK\r\nServer: Service Locator\r\nPragma: no-cache\r\nConnection: Close\r\nContent-Length: 12\r\nContent-Type: text/html\r\n\r\nPort=";
            string url = "http://localhost:3580/publish?" + _launchID + baseResponse + port + "\r\n";

            try
            {
                HttpResponseMessage response = _httpClient.GetAsync(Uri.EscapeUriString(url)).Result;
                _registered = true;
            }
            catch (Exception ex)
            {
                throw new ServiceLocatorRegistrationException("Exception trying to register port. Is NI Service Locator Service running?", ex);
            }


        }

        public string CreateLaunchID(string viPath, lvVersion lvVer)
        {
            //convert to full path since LabVIEW doesn't know our CWD
            string fullViPath = Path.GetFullPath(viPath);

            Regex forbiddenCharacters = new Regex(@"[:\\.\s]");
            string viPathEscaped = forbiddenCharacters.Replace(fullViPath, "");

            _launchID = "cli/" + lvVer.Version.Substring(0, 4) + '/' + lvVer.Bitness + '/' + viPathEscaped;

            return _launchID;
        }

        public void unRegister()
        {
            if(_registered)
            {
                _httpClient.GetAsync("http://localhost:3580/delete?" + _launchID);
            }

        }
    }

    public class ServiceLocatorRegistrationException : Exception
    {

        public ServiceLocatorRegistrationException()
        {

        }

        public ServiceLocatorRegistrationException(string message)
            : base(message)
        {

        }

        public ServiceLocatorRegistrationException(string message, Exception innerexception)
            : base(message, innerexception)
        {

        }
    }
}
