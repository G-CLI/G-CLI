using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using G_CLI;

namespace GCLI_Unit_tests
{
    [TestClass]
    public class LVLauncherTests
    {
        [TestMethod]
        public void TestLabVIEWArgumentsNoSuppression()
        {
            string arguments = LvLauncher.generateBaseArguments(1000, false);

            Assert.AreEqual("-- -p:1000", arguments);
        }

        [TestMethod]
        public void TestLabVIEWArgumentsWithDialogSuppression()
        {
            string arguments = LvLauncher.generateBaseArguments(1000, true);

            Assert.AreEqual("-unattended -- -p:1000", arguments);
        }
    }
}
