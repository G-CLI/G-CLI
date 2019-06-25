using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using G_CLI;

namespace GCLI_Unit_tests
{
    [TestClass]
    public class LVVersionUnitTests
    {
        [TestMethod]
        public void TestBuildsExePath()
        {
            lvVersion version;

            version = new lvVersion { Version = "2015", Path = "C:\\LabVIEW" };
            Assert.AreEqual(version.ExePath, "C:\\LabVIEW\\LabVIEW.exe");
        }

        [TestMethod]
        public void TestBuildsSearchPath()
        {
            lvVersion version;

            version = new lvVersion { Version = "2015", Path = "C:\\LabVIEW" };
            Assert.AreEqual(version.ToolsPath, "C:\\LabVIEW\\vi.lib\\G CLI Tools");
        }
    }
}
