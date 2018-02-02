using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using LabVIEW_CLI;

namespace LabVIEWCLI_Unit_tests
{
    [TestClass]
    public class ExitCodeUnitTests
    {

        private LabVIEW_CLI.lvComms lvCommsInstance;

        [TestInitialize]
        public void setUp()
        {
            lvCommsInstance = new lvComms();
            Console.WriteLine("Test123");

        }

        [TestCleanup]
        public void tearDown()
        {
            lvCommsInstance.Close();

        }

        [TestMethod]
        public void TestPositiveCode()
        {

            string msgData = "4587";

            Int32 exitCode = lvCommsInstance.extractExitCode(msgData);
            Assert.AreEqual(4587,exitCode);

        }

        [TestMethod]
        public void TestNegativeCode()
        {

            string msgData = "-4587";

            Int32 exitCode = lvCommsInstance.extractExitCode(msgData);
            Assert.AreEqual(-4587, exitCode);

        }
    }
}
