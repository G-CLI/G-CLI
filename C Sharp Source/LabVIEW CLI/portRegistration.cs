using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Net.Http;
using System.Text.RegularExpressions;

namespace LabVIEW_CLI
{
    class portRegistration
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

            

            Regex forbiddenCharacters = new Regex(@"[:\\.\s]");
            string viPathEscaped = forbiddenCharacters.Replace(viPath, "");

            _launchID = "cli/" + lvVer.Version.Substring(0,4) + '/' + lvVer.Bitness + '/' + viPathEscaped;

            string baseResponse = "=HTTP/1.0 200 OK\r\nServer: Service Locator\r\nPragma: no-cache\r\nConnection: Close\r\nContent-Length: 12\r\nContent-Type: text/html\r\n\r\nPort=";
            string url = "http://localhost:3580/publish?" + _launchID + baseResponse + port + "\r\n";

            HttpResponseMessage response = _httpClient.GetAsync(Uri.EscapeUriString(url)).Result;

            _registered = true;


        }

        public void unRegister()
        {
            if(_registered)
            {
                _httpClient.GetAsync("http://localhost:3580/delete?" + _launchID);
            }

        }
    }
}
