
using Microsoft.VisualStudio.TestTools.UnitTesting;
using G_CLI;
using System.IO;


namespace GCLI_Unit_tests
{
    [TestClass]
    public class Port_Registration_Unit_Tests
    {
        private G_CLI.portRegistration portRegistrationInstance;
        private lvVersion testVersion;


        [TestInitialize]
        public void setup()
        {
            portRegistrationInstance = new G_CLI.portRegistration();
            testVersion = new lvVersion();

        }

        [TestCleanup]
        public void tearDown()
        {
            //not needed?
        }

        [TestMethod]
        public void TestBuildsTheCorrectRegistrationID_32bit()
        {
            testVersion.Version = "2011 SP1 f2";
            testVersion.Bitness = "32bit";

            string result = portRegistrationInstance.CreateLaunchID("C:\\myVI.vi", testVersion);

            Assert.AreEqual("cli/2011/32bit/CmyVIvi", result);

        }

        [TestMethod]
        public void TestBuildsTheCorrectRegistrationID_64bit()
        {
            testVersion.Version = "2011 SP1 f2";
            testVersion.Bitness = "64bit";

            string result = portRegistrationInstance.CreateLaunchID("C:\\myVI.vi", testVersion);

            Assert.AreEqual("cli/2011/64bit/CmyVIvi", result);

        }

        [TestMethod]
        public void TestBuildsTheCorrectRegistrationID_ExtendsNameOnlyToFullPath()
        {
            string currentDirectory = Directory.GetCurrentDirectory();

            testVersion.Version = "2011 SP1 f2";
            testVersion.Bitness = "64bit";

            //fake current directory.= for test.
            Directory.SetCurrentDirectory("C:\\");
            string result = portRegistrationInstance.CreateLaunchID("myVI.vi", testVersion);

            //revert current directory.
            Directory.SetCurrentDirectory(currentDirectory);

            //Since we don't know the working directory, all we know is it shouldn't do this.
            Assert.AreEqual("cli/2011/64bit/CmyVIvi", result);

        }
    }
}
