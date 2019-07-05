using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using G_CLI;

namespace GCLI_Unit_tests
{
    [TestClass]
    public class ExitCodeUnitTests
    {

        private G_CLI.lvComms lvCommsInstance;

        [TestInitialize]
        public void setUp()
        {
            lvCommsInstance = new lvComms();

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

    [TestClass]
    public class MessageFormattingUnitTests
    {
        private G_CLI.lvComms lvCommsInstance;

        [TestInitialize]
        public void setUp()
        {
            lvCommsInstance = new lvComms();

        }

        [TestCleanup]
        public void tearDown()
        {
            lvCommsInstance.Close();

        }

        [TestMethod]
        public void testStandardCommandLineMessageFormatting()
        {

            string[] lvArgs = { "arg1", "arg2", "arg3" };

            lvMsg expectedMessage = new lvMsg("ARGS", "arg1\targ2\targ3");

            lvMsg messageToSend;

            messageToSend = lvCommsInstance.generateArgumentsMessage(lvArgs);

            Assert.AreEqual(expectedMessage.messageType, messageToSend.messageType);
            Assert.AreEqual(expectedMessage.messageData, messageToSend.messageData);
        }

    }
}
